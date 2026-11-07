import test from 'node:test';
import assert from 'node:assert/strict';
import { runAgent } from '../../.github/scripts/pr-explainer/agent.mjs';

test('large inspection batches reserve context for checks and publication', async () => {
  const phases = [];
  let checked = false;
  let published = false;
  const client = { chat: { completions: { create: async (request) => {
    assert.ok(JSON.stringify(request.messages).length <= 40000);
    const names = request.tools.map((tool) => tool.function.name);
    phases.push(names);
    const name = names.includes('get_file_diff') ? 'get_file_diff' : names[0];
    const args = name === 'get_file_diff' ? { path: 'seed.rs' }
      : name === 'publish_explanation' ? { body: 'Bounded explanation' } : {};
    return { choices: [{ finish_reason: 'tool_calls', message: {
      role: 'assistant', content: null,
      tool_calls: Array.from({ length: name === 'get_file_diff' ? 8 : 1 }, (_, i) => ({
        id: `${phases.length}-${i}`, type: 'function',
        function: { name, arguments: JSON.stringify(args) },
      })),
    } }] };
  } } } };
  await runAgent({ client, model: 'test', maxCharacters: 40000,
    messages: [{ role: 'system', content: 'Explain only inspected evidence' }],
    handlers: {
      get_file_diff: async () => ({ text: 'seed data'.repeat(3000) }),
      get_pr_checks: async () => { checked = true; return { status: 'success' }; },
      publish_explanation: async () => { assert.ok(checked); published = true; return { ok: true }; },
    },
  });
  assert.ok(published);
  assert.deepEqual(phases.slice(-2), [['get_pr_checks'], ['publish_explanation']]);
});
