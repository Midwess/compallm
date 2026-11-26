"""Profile the allocation fixture in an isolated Linux/GNU-linker debug build.

Run: python3 libs/codec/tests/profile_request_copies.py [--release]
Requires Python 3, Cargo, and a C compiler (CC, default cc).

Known payload writes and additional intercepted memcpy/memmove bytes are disjoint:
bulk hooks are suppressed around known operations. Additional bytes include library
and compiler-generated metadata moves, not just request payload. memset is separate.
Inlined copies and hidden libc calls are not fully observable. Instrumentation can
change compiler optimizations. These are instrumented-build observations, not an
exhaustive count of memory traffic or a timing benchmark.
Production source and the normal test build are not modified. Workspaces/logs remain
under /tmp/opencode for inspection. Existing allocation assertions still run.
"""

import ast
import argparse
import ctypes
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile


METER = r"""
pub mod copy_measure {
    unsafe extern "C" {
        fn codec_copy_stage(stage: u32);
        fn codec_copy_push();
        fn codec_copy_pop();
        fn codec_copy_reset();
        fn codec_copy_count(stage: u32, kind: u32) -> u64;
    }
    thread_local! {
        static COUNTS: std::cell::Cell<[usize; 10]> = const { std::cell::Cell::new([0; 10]) };
    }
    pub struct Suppress;
    impl Drop for Suppress {
        fn drop(&mut self) { unsafe { codec_copy_pop(); } }
    }
    pub fn suppress() -> Suppress {
        unsafe { codec_copy_push(); }
        Suppress
    }
    pub fn add(index: usize, length: usize) {
        let _guard = suppress();
        COUNTS.with(|counts| {
            let mut values = counts.get();
            values[index] += length;
            counts.set(values);
        });
    }
    pub fn stage(stage: u32) { unsafe { codec_copy_stage(stage); } }
    pub fn reset() {
        unsafe { codec_copy_reset(); }
        COUNTS.with(|counts| counts.set([0; 10]));
    }
    pub fn counts() -> ([usize; 10], [[u64; 3]; 2]) {
        let mut bulk = [[0; 3]; 2];
        for stage in 0..2 {
            for kind in 0..3 {
                bulk[stage][kind] = unsafe { codec_copy_count(stage as u32, kind as u32) };
            }
        }
        (COUNTS.with(|counts| counts.get()), bulk)
    }
}
"""


def instrument(workspace):
    def edit(relative, changes):
        path = workspace / relative
        text = path.read_text()
        for old, new in changes:
            if old not in text:
                raise RuntimeError(f"Instrumentation site changed: {relative}: {old}")
            text = text.replace(old, new)
        path.write_text(text)

    def known(index, expression, size):
        return ("{ let _copy_guard = compallm_serde_rules::copy_measure::suppress(); "
                f"compallm_serde_rules::copy_measure::add({index}, {size}); {expression} }}")

    path = workspace / "libs/serde-rules/src/lib.rs"
    path.write_text(path.read_text() + METER)
    edit("libs/codec/src/json/token.rs", [
        ("self.bytes.extend_from_slice(part);", known(0, "self.bytes.extend_from_slice(part);", "part.len()")),
        ("decoded.resize(bytes.len(), 0);", known(9, "decoded.resize(bytes.len(), 0);", "bytes.len().saturating_sub(decoded.len())")),
    ])
    edit("libs/codec/src/json/state.rs", [
        ("self.bytes.extend_from_slice(bytes);", known(1, "self.bytes.extend_from_slice(bytes);", "bytes.len()")),
    ])
    edit("libs/serde-rules/src/string.rs", [
        ("let mut source = 0;", "let _copy_guard = crate::copy_measure::suppress(); let mut source = 0;"),
        ("std::str::from_utf8(&output[..written])", "{ crate::copy_measure::add(2, written); std::str::from_utf8(&output[..written]) }"),
    ])
    edit("libs/codec/src/json/capture.rs", [
        ("self.text.push_str(value);", known(3, "self.text.push_str(value);", "value.len()")),
        ("self.text.push_str(&source.as_ref()[range.clone()]);", known(3, "self.text.push_str(&source.as_ref()[range.clone()]);", "range.len()")),
    ])
    edit("libs/codec/src/json/grammar.rs", [
        ("self.key_bytes.push_str(key);", known(4, "self.key_bytes.push_str(key);", "key.len()")),
    ])
    edit("libs/codec/src/json/encoder.rs", [
        ("output[..written].copy_from_slice(&remaining[..written]);", known(8, "output[..written].copy_from_slice(&remaining[..written]);", "written")),
        ("output[written] = byte;", known(6, "output[written] = byte;", "1")),
        ("self.output[self.written..self.written + count].copy_from_slice(&bytes[..count]);", known(6, "self.output[self.written..self.written + count].copy_from_slice(&bytes[..count]);", "count")),
        ("self.overflow.extend_from_slice(&bytes[count..]);", known(7, "self.overflow.extend_from_slice(&bytes[count..]);", "bytes.len() - count")),
    ])
    edit("libs/codec/tests/compatible_openai_full_request.rs", [
        ("fn measure_full_request_allocations<const BUFFER_SIZE: usize>() {", "fn measure_full_request_allocations<const BUFFER_SIZE: usize>() { compallm_serde_rules::copy_measure::reset();"),
        ('let status = decoder.next(&mut input, false).expect("decode component");', 'compallm_serde_rules::copy_measure::stage(1); let status = decoder.next(&mut input, false).expect("decode component"); compallm_serde_rules::copy_measure::stage(0);'),
        ("let status = encoder\n", "compallm_serde_rules::copy_measure::stage(2); let status = encoder\n"),
        ('.expect("encode decoded component");', '.expect("encode decoded component"); compallm_serde_rules::copy_measure::stage(0);'),
        ("let decoder_finished = matches!(", "compallm_serde_rules::copy_measure::stage(1); let decoder_finished = matches!("),
        ('decoder.finish().expect("finish decoding");', 'decoder.finish().expect("finish decoding"); compallm_serde_rules::copy_measure::stage(0);'),
        ('.expect("finalize encoding");', '.expect("finalize encoding"); compallm_serde_rules::copy_measure::stage(0);'),
        ('encoder.finish().expect("finish encoding");', 'compallm_serde_rules::copy_measure::stage(2); encoder.finish().expect("finish encoding"); compallm_serde_rules::copy_measure::stage(0);'),
        ("let total = measurement.snapshot();\n    drop(measurement);", 'let total = measurement.snapshot();\n    drop(measurement);\n    println!("COPY_PROFILE buffer={} input={} output={} stats={:?}", BUFFER_SIZE, REQUEST.len(), encoded_length, compallm_serde_rules::copy_measure::counts());'),
    ])


def check_hooks(library):
    hooks = ctypes.CDLL(str(library))
    hooks.codec_copy_count.argtypes = [ctypes.c_uint, ctypes.c_uint]
    hooks.codec_copy_count.restype = ctypes.c_uint64
    source = ctypes.create_string_buffer(b"abcdefghi")
    target = ctypes.create_string_buffer(16)
    hooks.codec_copy_reset()
    hooks.codec_copy_stage(1)
    for name in ["memcpy", "memmove", "memset"]:
        function = getattr(hooks, "__wrap_" + name)
        function.argtypes = [ctypes.c_void_p, ctypes.c_int if name == "memset" else ctypes.c_void_p, ctypes.c_size_t]
        function.restype = ctypes.c_void_p
        function(target, 0 if name == "memset" else source, 9)
    hooks.codec_copy_push()
    hooks.__wrap_memcpy(target, source, 9)
    hooks.codec_copy_pop()
    hooks.codec_copy_stage(0)
    assert [hooks.codec_copy_count(0, kind) for kind in range(3)] == [9, 9, 9]
    assert [hooks.codec_copy_count(1, kind) for kind in range(3)] == [0, 0, 0]


def report(log):
    rows = []
    for buffer, input_bytes, output_bytes, stats in re.findall(
        r"COPY_PROFILE buffer=(\d+) input=(\d+) output=(\d+) stats=(.*)", log
    ):
        known, bulk = ast.literal_eval(stats)
        input_bytes, output_bytes = int(input_bytes), int(output_bytes)
        decode = sum(known[:5])
        encode = sum(known[6:9])
        extra_decode, extra_encode = (sum(stage[:2]) for stage in bulk)
        rows.append({
            "buffer": int(buffer), "input": input_bytes, "output": output_bytes,
            "known_decode": decode, "known_encode": encode,
            "extra_bulk_decode": extra_decode, "extra_bulk_encode": extra_encode,
            "decode_ratio": (decode + extra_decode) / input_bytes,
            "encode_ratio": (encode + extra_encode) / output_bytes,
            "combined_ratio": (decode + encode + extra_decode + extra_encode) / (input_bytes + output_bytes),
            "known_zero_fill": known[9],
            "extra_memset_decode": bulk[0][2], "extra_memset_encode": bulk[1][2],
            "known_categories": known, "bulk_categories": bulk,
        })
    if len(rows) != 4:
        raise RuntimeError(f"Expected four buffer cases, found {len(rows)}")
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--release", action="store_true", help="profile optimized code instead of the default debug build")
    args = parser.parse_args()
    if sys.platform != "linux":
        raise SystemExit("This optional profiler requires Linux and a GNU-compatible linker.")
    repository = Path(__file__).resolve().parents[3]
    base = Path("/tmp/opencode")
    base.mkdir(exist_ok=True)
    workspace = Path(tempfile.mkdtemp(prefix="codec-copy-profile-", dir=base))
    for relative in ["libs/codec", "libs/serde-rules"]:
        shutil.copytree(repository / relative, workspace / relative,
                        ignore=shutil.ignore_patterns("target", "__pycache__"))
    manifest = (repository / "Cargo.toml").read_text()
    manifest, replacements = re.subn(r"members = \[.*?\]", 'members = ["libs/codec", "libs/serde-rules"]', manifest, count=1)
    if replacements != 1:
        raise RuntimeError("Workspace members declaration changed")
    (workspace / "Cargo.toml").write_text(manifest)
    shutil.copy2(repository / "Cargo.lock", workspace / "Cargo.lock")
    instrument(workspace)
    library = workspace / "libcodec_copy_profile.so"
    subprocess.run(shlex.split(os.environ.get("CC", "cc")) + [
        "-shared", "-fPIC", "-O2", "-fno-builtin", "-Wl,-Bsymbolic-functions",
        str(repository / "libs/codec/tests/support/copy_profile.c"), "-o", str(library),
    ], check=True)
    check_hooks(library)
    env = os.environ.copy()
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    env["RUSTFLAGS"] = (env.get("RUSTFLAGS", "") + f" -L native={workspace} -l dylib=codec_copy_profile"
                        " -C link-arg=-Wl,--wrap=memcpy,--wrap=memmove,--wrap=memset")
    env["LD_LIBRARY_PATH"] = str(workspace) + ":" + env.get("LD_LIBRARY_PATH", "")
    # Preload only the test executable, not Cargo/rustc. Linker wrapping covers
    # static Rust dependencies; preloading also covers interposable shared calls.
    command = [
        "cargo", "test", "-p", "codec", "--test", "compatible_openai_full_request",
        "--no-run", "--message-format=json",
    ]
    if args.release:
        command.append("--release")
    result = subprocess.run(command, cwd=workspace, env=env, capture_output=True, text=True)
    (workspace / "build.log").write_text(result.stdout + result.stderr)
    if result.returncode:
        raise RuntimeError(f"Build failed; see {workspace / 'build.log'}")
    executable = None
    for line in result.stdout.splitlines():
        message = json.loads(line)
        if message.get("reason") == "compiler-artifact" and message.get("executable"):
            executable = message["executable"]
    if not executable:
        raise RuntimeError("Cargo did not report a test executable")
    env["LD_PRELOAD"] = str(library) + (":" + env["LD_PRELOAD"] if env.get("LD_PRELOAD") else "")
    result = subprocess.run([executable, "measures_", "--nocapture", "--test-threads=1"],
                            cwd=workspace, env=env, capture_output=True, text=True)
    (workspace / "test.log").write_text(result.stdout + result.stderr)
    if result.returncode:
        raise RuntimeError(f"Test failed; see {workspace / 'test.log'}")
    rows = report(result.stdout)
    for row in rows:
        row["profile"] = "release" if args.release else "debug"
    (workspace / "results.json").write_text(json.dumps(rows, indent=2) + "\n")
    print(f"Profile: {'release' if args.release else 'debug'}; ratio denominator = input + output")
    print("buffer | input | output | known decode | extra bulk decode | known encode | extra bulk encode | combined ratio")
    for row in rows:
        print(f"{row['buffer']} | {row['input']} | {row['output']} | {row['known_decode']} | {row['extra_bulk_decode']} | {row['known_encode']} | {row['extra_bulk_encode']} | {row['combined_ratio']:.2%}")
    print("Extra bulk bytes include library/compiler metadata moves; inlined/hidden copies can be missed.")
    print("Buffer initialization/memset bytes are separate in results.json. Harness copies and construction are excluded.")
    print(f"Artifacts: {workspace}")


if __name__ == "__main__":
    main()
