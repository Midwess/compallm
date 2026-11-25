use serde::de::Error;

pub fn decode<'a>(input: &'a [u8], output: &'a mut [u8]) -> Result<&'a str, serde_json::Error> {
    if input.first() != Some(&b'"') || input.last() != Some(&b'"') || input.len() < 2 {
        return Err(serde_json::Error::custom("invalid JSON string"));
    }
    let bytes = &input[1..input.len() - 1];
    if !bytes.contains(&b'\\') {
        if bytes.iter().any(|byte| *byte < 0x20 || *byte == b'"') {
            return Err(serde_json::Error::custom("invalid JSON string"));
        }
        return std::str::from_utf8(bytes).map_err(serde_json::Error::custom);
    }
    let mut source = 0;
    let mut written = 0;
    while source < bytes.len() {
        let byte = bytes[source];
        source += 1;
        if byte < 0x20 || byte == b'"' {
            return Err(serde_json::Error::custom("invalid JSON string"));
        }
        if byte != b'\\' {
            *output
                .get_mut(written)
                .ok_or_else(|| serde_json::Error::custom("decoded string storage exhausted"))? =
                byte;
            written += 1;
            continue;
        }
        let escape = *bytes
            .get(source)
            .ok_or_else(|| serde_json::Error::custom("incomplete JSON escape"))?;
        source += 1;
        let decoded = match escape {
            b'"' => b'"',
            b'\\' => b'\\',
            b'/' => b'/',
            b'b' => 8,
            b'f' => 12,
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'u' => {
                let first = hex(bytes, &mut source)?;
                let scalar = match first {
                    0xD800..=0xDBFF => {
                        if bytes.get(source..source + 2) != Some(b"\\u") {
                            return Err(serde_json::Error::custom("invalid Unicode surrogate"));
                        }
                        source += 2;
                        let second = hex(bytes, &mut source)?;
                        if !(0xDC00..=0xDFFF).contains(&second) {
                            return Err(serde_json::Error::custom("invalid Unicode surrogate"));
                        }
                        0x10000 + ((u32::from(first) - 0xD800) << 10) + (u32::from(second) - 0xDC00)
                    }
                    0xDC00..=0xDFFF => {
                        return Err(serde_json::Error::custom("invalid Unicode surrogate"));
                    }
                    _ => u32::from(first),
                };
                let character = char::from_u32(scalar)
                    .ok_or_else(|| serde_json::Error::custom("invalid Unicode scalar"))?;
                let mut encoded = [0; 4];
                let text = character.encode_utf8(&mut encoded);
                output
                    .get_mut(written..written + text.len())
                    .ok_or_else(|| serde_json::Error::custom("decoded string storage exhausted"))?
                    .copy_from_slice(text.as_bytes());
                written += text.len();
                continue;
            }
            _ => return Err(serde_json::Error::custom("invalid JSON escape")),
        };
        *output
            .get_mut(written)
            .ok_or_else(|| serde_json::Error::custom("decoded string storage exhausted"))? =
            decoded;
        written += 1;
    }
    std::str::from_utf8(&output[..written]).map_err(serde_json::Error::custom)
}

fn hex(bytes: &[u8], position: &mut usize) -> Result<u16, serde_json::Error> {
    let digits = bytes
        .get(*position..*position + 4)
        .ok_or_else(|| serde_json::Error::custom("incomplete Unicode escape"))?;
    let mut value = 0;
    for byte in digits {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return Err(serde_json::Error::custom("invalid Unicode escape")),
        };
        value = (value << 4) | u16::from(digit);
    }
    *position += 4;
    Ok(value)
}
