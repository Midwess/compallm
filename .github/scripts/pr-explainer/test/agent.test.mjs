import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import OpenAI from 'openai';
import { Cache, MARKER, Publisher, hash, runAgent, validateArgs } from '../agent.mjs';
import { validateExplanation } from '../diagram.mjs';
import { main } from '../index.mjs';

const body = `## Idea
Reject oversized registration fields before database access.

## What changes
- Display names and emails have a maximum of 256 characters.

## Diagram
\`\`\`mermaid
flowchart LR
  A["Registration"] --> B{"Fields valid?"}
  B -->|Yes| C["Create user"]
  B -->|No| D["Reject input"]
\`\`\`

## Review focus
Check that limits count Unicode characters consistently.

## Checks / coverage
Checks are pending. No tests were run by this assistant.`;

test('cache persists immutable inputs, coalesces concurrent reads, and retries failures', async t => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'pr-explainer-test-'));
  t.after(() => rm(directory, { recursive: true, force: true }));
  const cache = new Cache(directory);
  let reads = 0;
  const key = ['file', 'owner/repo', 'sha', 'file.rs'];
  const compute = async () => { reads++; return 'content'; };
  assert.deepEqual(await Promise.all([cache.remember(key, compute), cache.remember(key, compute)]), ['content', 'content']);
  assert.equal(reads, 1);
  assert.equal(await new Cache(directory).remember(key, compute), 'content');
  assert.equal(reads, 1);
  assert.notEqual(hash([...key, 'new-sha']), hash(key));
  await assert.rejects(cache.remember(['failure'], async () => { throw new Error('temporary'); }));
  assert.equal(await cache.remember(['failure'], async () => 'recovered'), 'recovered');
});

test('tool arguments reject arbitrary commands, paths, revisions, and oversized line ranges', () => {
  validateArgs('read_file', { path: 'libs/core/file.rs', revision: 'head', start_line: 1, end_line: 300 });
  for (const args of [
    { path: '../secret', revision: 'head', start_line: 1, end_line: 2 },
    { path: '/etc/passwd', revision: 'head', start_line: 1, end_line: 2 },
    { path: 'file.rs', revision: 'other-repo', start_line: 1, end_line: 2 },
    { path: 'file.rs', revision: 'head', start_line: 1, end_line: 301 },
    { path: 'file.rs', revision: 'head', start_line: 2, end_line: 1 },
    { path: 'file.rs', revision: 'head', start_line: 1, end_line: 2, command: 'rm -rf /' },
  ]) assert.throws(() => validateArgs('read_file', args));
  assert.throws(() => validateArgs('shell', { command: 'gh api repos/other/repo' }));
});

test('explanation validates Mermaid syntax and rejects unsafe or malformed content', async () => {
  assert.equal(await validateExplanation(body), body.trim());
  await assert.rejects(validateExplanation(body.replace('A["Registration"]', 'A["unclosed"')));
  await assert.rejects(validateExplanation(body.replace('flowchart LR', '%%{init: {}}%%\nflowchart LR')));
  await assert.rejects(validateExplanation(body.replace(/flowchart LR[\s\S]*?```/, 'sequenceDiagram\n    A->>B: Request\n```')), /flowchart/);
  await assert.rejects(validateExplanation(`${body}\n<!-- forged marker -->`));
  await assert.rejects(validateExplanation(`${body}\n@someone`));
  await assert.rejects(validateExplanation(body.replace('## Idea', '## Missing')));
  await assert.rejects(validateExplanation(`${body}\n${'extra '.repeat(351)}`), /at most 350/);
  assert.ok(await validateExplanation(`${body}\nExample: user@example.com and \`Option<User>\`.`));
  await assert.rejects(validateExplanation(`${body}\n<div>HTML</div>`), /HTML tags/);
  await assert.rejects(validateExplanation(body.replace('```\n\n## Review', '\n\n## Review')));
});

const pr = { state: 'open', draft: false, base: { sha: 'base' }, head: { sha: 'head' } };
function fixture({ existing = [], snapshots = [pr] } = {}) {
  const writes = [];
  const saved = [];
  let reads = 0;
  const api = async (endpoint, options = {}) => {
    if (options.method) { writes.push({ endpoint, ...options }); return { id: 42 }; }
    if (endpoint.includes('/comments')) return [existing];
    return snapshots[Math.min(reads++, snapshots.length - 1)];
  };
  const publisher = new Publisher({ api, repo: 'owner/repo', number: 1, base: 'base', head: 'head',
    cache: { put: async (...args) => saved.push(args) }, key: 'key', validate: validateExplanation });
  return { publisher, writes, saved };
}

test('publisher requires check evidence, creates once, and updates only its own comment', async () => {
  const first = fixture();
  await assert.rejects(first.publisher.publish(body), /get_pr_checks/);
  assert.equal(first.writes.length, 0);
  first.publisher.checksRead = true;
  await first.publisher.publish(body);
  assert.equal(first.writes[0].method, 'POST');
  assert.match(first.writes[0].body.body, /analysis:key/);
  assert.equal(first.saved.length, 1);

  const human = { id: 2, user: { login: 'contributor', type: 'User' }, body: MARKER };
  const bot = { id: 3, user: { login: 'github-actions[bot]', type: 'Bot' }, body: MARKER };
  const next = fixture({ existing: [human, bot] });
  next.publisher.checksRead = true;
  await next.publisher.publish(body);
  assert.equal(next.writes[0].method, 'PATCH');
  assert.equal(next.writes[0].endpoint, 'repos/owner/repo/issues/comments/3');
});

test('publication refuses stale commits, changed bases, drafts, and closed PRs without caching output', async () => {
  for (const changed of [
    { ...pr, head: { sha: 'new-head' } }, { ...pr, base: { sha: 'new-base' } },
    { ...pr, draft: true }, { ...pr, state: 'closed' },
  ]) {
    // Simulate a change while fetching existing comments.
    const result = fixture({ snapshots: [pr, changed] });
    result.publisher.checksRead = true;
    await assert.rejects(result.publisher.publish(body), /stale explanation/);
    assert.equal(result.writes.length, 0);
    assert.equal(result.saved.length, 0);
  }
});

const call = (id, name, args = {}) => ({ id, type: 'function', function: { name, arguments: JSON.stringify(args) } });
const completion = calls => ({ choices: [{ finish_reason: 'tool_calls', message: {
  role: 'assistant', content: null, reasoning_details: [{ type: 'text', text: 'provider field' }], tool_calls: calls,
} }] });

test('real OpenAI SDK uses configured endpoint/model and preserves tool-call IDs and provider fields', async () => {
  const requests = [];
  const replies = [
    completion([call('read-1', 'read_file', { path: 'file.rs', revision: 'head', start_line: 1, end_line: 4 })]),
    completion([call('checks-1', 'get_pr_checks'), call('publish-1', 'publish_explanation', { body })]),
  ];
  const client = new OpenAI({ apiKey: 'test-key', baseURL: 'https://compatible.example/v1', maxRetries: 0,
    fetch: async (url, options) => {
      assert.equal(String(url), 'https://compatible.example/v1/chat/completions');
      assert.equal(new Headers(options.headers).get('authorization'), 'Bearer test-key');
      requests.push(JSON.parse(options.body));
      return new Response(JSON.stringify(replies.shift()), { headers: { 'content-type': 'application/json' } });
    },
  });
  let checked = false;
  let published = 0;
  await runAgent({ client, model: 'environment-model', messages: [{ role: 'system', content: 'Explain PR' }], handlers: {
    read_file: async () => ({ text: '1: code' }),
    get_pr_checks: async () => { checked = true; return { status: 'pending' }; },
    publish_explanation: async args => { assert.ok(checked); await validateExplanation(args.body); published++; return { ok: true }; },
  } });
  assert.equal(published, 1);
  assert.equal(requests[0].model, 'environment-model');
  assert.equal(requests[1].messages[2].tool_call_id, 'read-1');
  assert.equal(requests[1].messages[1].reasoning_details[0].text, 'provider field');
});

test('invalid model tools return errors for correction and do not execute handlers', async () => {
  let step = 0;
  let invoked = 0;
  const messages = [];
  const client = { chat: { completions: { create: async () => step++ === 0
    ? completion([call('bad', 'shell', { command: 'echo secret' })])
    : completion([call('done', 'publish_explanation', { body })]),
  } } };
  await runAgent({ client, model: 'model', messages, handlers: {
    shell: async () => { invoked++; }, publish_explanation: async () => ({ ok: true }),
  } });
  assert.equal(invoked, 0);
  assert.match(messages[1].content, /Unknown tool/);
});

test('tool and context budgets prevent unbounded model requests', async () => {
  let requests = 0;
  const client = { chat: { completions: { create: async () => { requests++; return completion([call('read', 'get_pr_diff')]); } } } };
  await assert.rejects(runAgent({ client, model: 'model', messages: [], handlers: { get_pr_diff: async () => 'diff' }, maxRounds: 2 }), /conversation limit/);
  assert.equal(requests, 2);
  await assert.rejects(runAgent({ client, model: 'model', messages: [{ role: 'user', content: 'large' }], handlers: {}, maxCharacters: 1 }), /context budget/);
  assert.equal(requests, 2);
});

test('exploration cannot consume the rounds reserved for checks, publication, and validation repair', async () => {
  const available = [];
  let attempts = 0;
  const client = { chat: { completions: { create: async request => {
    const names = request.tools.map(tool => tool.function.name);
    available.push(names);
    if (names.includes('read_file')) return completion([call('diff', 'get_pr_diff')]);
    assert.equal(request.tool_choice, 'required');
    return completion([call('finalize', names[0], names[0] === 'publish_explanation' ? { body } : {})]);
  } } } };
  await runAgent({ client, model: 'model', messages: [], maxRounds: 5, handlers: {
    get_pr_diff: async () => 'diff', get_pr_checks: async () => ({ status: 'pending' }),
    publish_explanation: async () => { if (++attempts === 1) throw new Error('Correct the diagram'); return { ok: true }; },
  } });
  assert.equal(available.length, 5);
  assert.deepEqual(available.slice(-3), [['get_pr_checks'], ['publish_explanation'], ['publish_explanation']]);
  assert.equal(attempts, 2);
});

test('default budget allows twenty rounds with the last three reserved for completion', async () => {
  const phases = [];
  const client = { chat: { completions: { create: async request => {
    const names = request.tools.map(tool => tool.function.name);
    phases.push(names);
    const name = names.includes('get_pr_diff') ? 'get_pr_diff' : names[0];
    return completion([call('next', name, name === 'publish_explanation' ? { body } : {})]);
  } } } };
  let attempts = 0;
  await runAgent({ client, model: 'model', messages: [], handlers: {
    get_pr_diff: async () => 'diff', get_pr_checks: async () => ({ status: 'pending' }),
    publish_explanation: async () => { if (++attempts === 1) throw new Error('Correct draft'); return { ok: true }; },
  } });
  assert.equal(phases.length, 20);
  assert.equal(phases.slice(0, 17).filter(names => names.includes('read_file')).length, 17);
  assert.deepEqual(phases.slice(-3), [['get_pr_checks'], ['publish_explanation'], ['publish_explanation']]);
});

test('runner publishes once, skips matching inputs, restores a deleted comment, and refreshes changed PR metadata', async t => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'pr-explainer-runner-'));
  const savedEnv = { ...process.env };
  t.after(async () => {
    process.env = savedEnv;
    await rm(directory, { recursive: true, force: true });
  });
  const head = 'a'.repeat(40);
  const base = 'b'.repeat(40);
  const snapshot = { number: 1, state: 'open', draft: false, title: 'Validate registration', body: 'Limit input lengths',
    changed_files: 1, head: { sha: head, repo: { full_name: 'owner/repo' } }, base: { sha: base } };
  const eventFile = path.join(directory, 'event.json');
  const architectureFile = path.join(directory, 'architecture.md');
  await writeFile(eventFile, JSON.stringify({ pull_request: snapshot }));
  await writeFile(architectureFile, 'Business rules live in the domain.');
  Object.assign(process.env, { GITHUB_REPOSITORY: 'owner/repo', GITHUB_EVENT_PATH: eventFile,
    LLM_BASE_URL: 'https://compatible.example/v1', LLM_MODEL: 'configured-model', LLM_API_KEY: 'never-cache-this-key',
    GH_TOKEN: 'never-cache-this-token', EXPLAINER_CACHE_DIR: path.join(directory, 'cache') });
  let comment;
  let modelCalls = 0;
  let diffReads = 0;
  let fileReads = 0;
  let checkReads = 0;
  const request = async (endpoint, options = {}) => {
    if (options.method) {
      comment = { id: 1, body: options.body.body, user: { login: 'github-actions[bot]', type: 'Bot' } };
      return comment;
    }
    if (endpoint.includes('/comments')) return [comment ? [comment] : []];
    if (endpoint.includes('/pulls/')) return snapshot;
    if (endpoint.includes('/compare/')) {
      diffReads++;
      return options.raw ? 'diff --git a/file.rs b/file.rs\n+validate();' : {
        merge_base_commit: { sha: base }, files: [{ filename: 'file.rs', status: 'modified', patch: '+validate();' }],
      };
    }
    if (endpoint.includes('/contents/')) {
      assert.match(endpoint, new RegExp(`ref=${head}$`));
      fileReads++;
      return { type: 'file', encoding: 'base64', size: 11, content: Buffer.from('validate();').toString('base64') };
    }
    if (endpoint.includes('/check-runs')) { checkReads++; return [{ check_runs: [{ name: 'test', status: 'completed', conclusion: 'success' }] }]; }
    if (endpoint.endsWith('/status')) return { state: 'success', statuses: [] };
    throw new Error(`Unexpected endpoint ${endpoint}`);
  };
  const createClient = options => {
    assert.equal(options.baseURL, 'https://compatible.example/v1');
    let turn = 0;
    return { chat: { completions: { create: async () => {
      modelCalls++;
      return turn++ === 0 ? completion([
        call('read', 'read_file', { path: 'file.rs', revision: 'head', start_line: 1, end_line: 10 }),
        call('checks', 'get_pr_checks'),
      ]) : completion([call('publish', 'publish_explanation', { body })]);
    } } } };
  };
  const run = () => main({ request, createClient, architectureFile });
  await run();
  assert.ok(comment.body.startsWith(MARKER));
  assert.equal(modelCalls, 2);
  await run();
  assert.equal(modelCalls, 2);
  comment = undefined;
  await run();
  assert.ok(comment);
  assert.equal(modelCalls, 2);
  assert.equal(checkReads, 1);
  snapshot.title = 'Updated intent';
  await run();
  assert.equal(modelCalls, 4);
  assert.equal(diffReads, 2); // Comparison and full diff fetched only on the first run.
  assert.equal(fileReads, 1);
  assert.equal(checkReads, 2); // Checks are never restored from the file cache.
  await writeFile(eventFile, JSON.stringify({ inputs: { pr_number: '1' } }));
  await run(); // Manual dispatch supports an existing PR, including cache hits.
  assert.equal(modelCalls, 4);
  await writeFile(eventFile, JSON.stringify({ inputs: { pr_number: 'not-a-number' } }));
  await assert.rejects(run(), /pull-request event is required/);
});
