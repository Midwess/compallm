import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile, rename } from 'node:fs/promises';
import path from 'node:path';

export const hash = value => createHash('sha256').update(JSON.stringify(value)).digest('hex');
export const MARKER = '<!-- ai-pr-explanation -->';
export const clip = (text, limit) => ({ text: text.slice(0, limit), truncated: text.length > limit, totalCharacters: text.length });
export class StalePullRequest extends Error {}

export class Cache {
  constructor(directory) { this.directory = directory; this.pending = new Map(); }
  file(key) { return path.join(this.directory, `${hash(key)}.json`); }
  async get(key) {
    try { return JSON.parse(await readFile(this.file(key), 'utf8')); }
    catch (error) { if (error.code === 'ENOENT' || error instanceof SyntaxError) return undefined; throw error; }
  }
  async put(key, value) {
    await mkdir(this.directory, { recursive: true });
    const file = this.file(key);
    const temporary = `${file}.${process.pid}.tmp`;
    await writeFile(temporary, JSON.stringify(value), { mode: 0o600 });
    await rename(temporary, file);
    return value;
  }
  async remember(key, compute) {
    const id = hash(key);
    if (!this.pending.has(id)) this.pending.set(id, (async () => {
      const existing = await this.get(key);
      return existing === undefined ? this.put(key, await compute()) : existing;
    })().catch(error => { this.pending.delete(id); throw error; }));
    return this.pending.get(id);
  }
}

const tool = (name, description, properties = {}, required = []) => ({
  type: 'function', function: { name, description, parameters: { type: 'object', properties, required, additionalProperties: false } },
});
export const tools = [
  tool('get_pr_diff', 'Read the immutable PR diff. Large output is explicitly truncated.'),
  tool('get_file_diff', 'Read the patch for a changed file, useful when the initial diff is truncated.',
    { path: { type: 'string' } }, ['path']),
  tool('read_file', 'Read code at a pinned base/head commit. Output has line numbers; request further ranges as needed.', {
    path: { type: 'string' }, revision: { type: 'string', enum: ['base', 'head'] },
    start_line: { type: 'integer', minimum: 1 }, end_line: { type: 'integer', minimum: 1 },
  }, ['path', 'revision', 'start_line', 'end_line']),
  tool('get_pr_checks', 'Read current check runs and commit statuses at the analyzed head.'),
  tool('publish_explanation', 'Validate and publish the final Markdown as one bot-owned PR comment.',
    { body: { type: 'string' } }, ['body']),
];

export function validateArgs(name, args) {
  const schema = tools.find(tool => tool.function.name === name)?.function.parameters;
  if (!schema || !args || typeof args !== 'object' || Array.isArray(args)) throw new Error('Unknown tool or invalid arguments.');
  if (Object.keys(args).some(key => !(key in schema.properties)) || schema.required.some(key => !(key in args))) {
    throw new Error('Unexpected or missing tool arguments.');
  }
  for (const [key, value] of Object.entries(args)) {
    const spec = schema.properties[key];
    if ((spec.type === 'integer' ? !Number.isSafeInteger(value) || value < spec.minimum : typeof value !== spec.type)
      || (spec.enum && !spec.enum.includes(value))) throw new Error(`Invalid argument: ${key}`);
  }
  if (args.path && (args.path.length > 1024 || args.path.startsWith('/') || args.path.split('/').some(p => !p || p === '.' || p === '..') || /[\x00-\x1f\\]/.test(args.path))) {
    throw new Error('Use a relative repository file path without traversal.');
  }
  if (name === 'read_file' && (args.end_line < args.start_line || args.end_line - args.start_line >= 300)) {
    throw new Error('Read between 1 and 300 lines per call.');
  }
}

export async function runAgent({ client, model, messages, handlers, maxRounds = 10, maxCalls = 40,
  maxCharacters = 240000, onProgress = () => {} }) {
  let calls = 0;
  let published = false;
  for (let round = 0; round < maxRounds; round++) {
    onProgress(`LLM round ${round + 1}/${maxRounds}`);
    // Reserve rounds for current checks, publication, and one validation repair.
    const finalizing = maxRounds >= 4 && round >= maxRounds - 3;
    const allowed = finalizing ? tools.filter(tool => tool.function.name ===
      (round === maxRounds - 3 ? 'get_pr_checks' : 'publish_explanation')) : tools;
    if (finalizing) messages.push({ role: 'user', content: round === maxRounds - 3
      ? 'Code inspection is complete. Read current checks now, then publish a concise explanation from the evidence gathered. Disclose any uninspected areas.'
      : 'Publish now using publish_explanation. Keep all prose under 350 words, aiming for 250; put code types in backticks and include no GitHub mentions. Use short bullets and one sentence for coverage. Correct ALL previous validation errors. Do not inspect more code.' });
    if (JSON.stringify(messages).length > maxCharacters) throw new Error('LLM context budget exceeded; no further request sent.');
    const response = await client.chat.completions.create({ model, messages, tools: allowed, tool_choice: finalizing ? 'required' : 'auto' });
    const choice = response.choices?.[0];
    const message = choice?.message;
    if (!message || choice.finish_reason === 'length' || choice.finish_reason === 'content_filter') {
      throw new Error('LLM returned an incomplete response.');
    }
    // Preserve provider-returned fields (including reasoning fields) for compatible endpoints.
    messages.push(message);
    const requested = message.tool_calls ?? [];
    onProgress(`Response: ${requested.length} tool calls (finish: ${choice.finish_reason})`);
    if (!requested.length) {
      messages.push({ role: 'user', content: 'Use the supplied tools to inspect the PR and publish_explanation to finish.' });
      continue;
    }
    if (requested.length > 8 || calls + requested.length > maxCalls) throw new Error('LLM tool-call budget exceeded.');
    calls += requested.length;
    // Read calls can run concurrently, but publication always follows all reads in this batch.
    const invoke = async call => {
      try {
        if (call.type !== 'function') throw new Error('Only function tools are available.');
        const args = JSON.parse(call.function.arguments);
        validateArgs(call.function.name, args);
        if (!allowed.some(tool => tool.function.name === call.function.name)) throw new Error('Inspection budget is exhausted; use the currently available tool.');
        onProgress(`Executing tool: ${call.function.name}`);
        if (published) return { ok: true, already_published: true };
        const result = await handlers[call.function.name](args);
        if (call.function.name === 'publish_explanation') published = true;
        return result;
      } catch (error) {
        if (error instanceof StalePullRequest) throw error;
        onProgress(`Tool failed: ${call.function?.name ?? 'unknown'}: ${error instanceof SyntaxError ? 'Invalid JSON' : error.message}`);
        // Tool implementations expose only safe errors; never return raw CLI/provider errors.
        return { error: error instanceof SyntaxError ? 'Arguments must be valid JSON.' : error.message };
      }
    };
    const results = await Promise.all(requested.map(call => call.function?.name === 'publish_explanation' ? null : invoke(call)));
    for (let i = 0; i < requested.length; i++) {
      const call = requested[i];
      if (call.function?.name === 'publish_explanation') results[i] = await invoke(call);
      messages.push({ role: 'tool', tool_call_id: call.id, content: JSON.stringify(results[i]) });
    }
    if (published) return;
  }
  throw new Error('LLM did not publish within the conversation limit.');
}

export class Publisher {
  constructor({ api, repo, number, base, head, metadata, cache, key, validate }) {
    Object.assign(this, { api, repo, number, base, head, metadata, cache, key, validate });
    this.checksRead = false;
  }
  async current() {
    const pr = await this.api(`repos/${this.repo}/pulls/${this.number}`);
    if (pr.state !== 'open' || pr.draft || pr.base.sha !== this.base || pr.head.sha !== this.head
      || (this.metadata && (pr.title !== this.metadata.title || pr.body !== this.metadata.body))) {
      throw new StalePullRequest('PR changed or is no longer ready; skipping stale explanation.');
    }
    return pr;
  }
  async comments() {
    return (await this.api(`repos/${this.repo}/issues/${this.number}/comments?per_page=100`, { paginate: true })).flat();
  }
  owned(comment) {
    return comment.user?.login === 'github-actions[bot]' && comment.user?.type === 'Bot' && comment.body?.startsWith(MARKER);
  }
  async existing() { return (await this.comments()).find(comment => this.owned(comment)); }
  async publish(body, { cached = false } = {}) {
    body = await this.validate(body);
    if (!cached && !this.checksRead) throw new Error('Call get_pr_checks before publishing.');
    await this.current();
    const existing = await this.existing();
    const rendered = `${MARKER}\n<!-- analysis:${this.key} -->\n${body}\n\n---\nAI-generated explanation · Analyzed commit [${this.head.slice(0, 12)}](https://github.com/${this.repo}/commit/${this.head}). Check results reflect analysis time.`;
    // Fetching comments can take time; check again immediately before the write.
    await this.current();
    if (existing) {
      await this.api(`repos/${this.repo}/issues/comments/${existing.id}`, { method: 'PATCH', body: { body: rendered } });
    } else {
      await this.api(`repos/${this.repo}/issues/${this.number}/comments`, { method: 'POST', body: { body: rendered } });
    }
    // Only reuse complete, successfully published explanations.
    await this.cache.put(['explanation', this.key], body);
    return { ok: true };
  }
}
