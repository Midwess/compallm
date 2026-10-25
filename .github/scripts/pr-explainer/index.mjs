import OpenAI from 'openai';
import { execFile } from 'node:child_process';
import { readFile, readdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { Cache, Publisher, clip, hash, runAgent, tools } from './agent.mjs';
import { validateExplanation } from './diagram.mjs';

const directory = path.dirname(fileURLToPath(import.meta.url));

export function gh(args, { body, raw = false } = {}) {
  return new Promise((resolve, reject) => {
    // Neither LLM secrets nor the rest of the workflow environment reach gh.
    const child = execFile('gh', args, {
      env: {
        PATH: process.env.PATH, HOME: process.env.HOME, GH_TOKEN: process.env.GH_TOKEN,
        GH_HOST: 'github.com', GH_PROMPT_DISABLED: '1', GH_PAGER: 'cat',
      },
      timeout: 45000, maxBuffer: 8 * 1024 * 1024,
    }, (error, stdout) => {
      if (error) return reject(new Error('GitHub request failed or exceeded its output/time limit.'));
      try { resolve(raw ? stdout : JSON.parse(stdout)); }
      catch { reject(new Error('GitHub returned invalid JSON.')); }
    });
    child.stdin.on('error', () => {}); // The process may exit before consuming stdin.
    child.stdin.end(body ? JSON.stringify(body) : undefined);
  });
}

export function api(endpoint, { method = 'GET', body, paginate = false, raw = false } = {}) {
  const args = ['api', '--hostname', 'github.com', '--method', method,
    '-H', `Accept: ${raw ? 'application/vnd.github.diff' : 'application/vnd.github+json'}`, endpoint];
  if (paginate) args.push('--paginate', '--slurp');
  if (body) args.push('--input', '-');
  return gh(args, { body, raw });
}

async function implementationDigest() {
  const names = (await readdir(directory)).filter(name => /\.(mjs|md|json)$/.test(name)).sort();
  return hash(await Promise.all(names.map(async name => [name, await readFile(path.join(directory, name), 'utf8')])));
}

export async function main({ request = api, createClient = options => new OpenAI(options),
  architectureFile = path.resolve(directory, '../../../docs/content/architecture.md') } = {}) {
  const api = request;
  const repo = process.env.GITHUB_REPOSITORY;
  if (!/^[\w.-]+\/[\w.-]+$/.test(repo ?? '')) throw new Error('Invalid GITHUB_REPOSITORY.');
  const event = JSON.parse(await readFile(process.env.GITHUB_EVENT_PATH, 'utf8'));
  const manualNumber = /^[1-9]\d*$/.test(event.inputs?.pr_number ?? '') ? Number(event.inputs.pr_number) : NaN;
  const number = event.pull_request?.number ?? manualNumber;
  if (!Number.isSafeInteger(number) || number < 1) throw new Error('A pull-request event is required.');
  const pr = await api(`repos/${repo}/pulls/${number}`);
  const expected = event.pull_request ?? pr;
  if (pr.state !== 'open' || pr.draft || pr.head.repo?.full_name !== repo
    || pr.head.sha !== expected.head.sha || pr.base.sha !== expected.base.sha) {
    console.log('Skipped: PR is a fork, draft, closed, or has changed since this event.');
    return;
  }
  const base = pr.base.sha;
  const head = pr.head.sha;
  if (![base, head].every(sha => /^[a-f0-9]{40}$/.test(sha))) throw new Error('Invalid commit SHA.');
  for (const name of ['LLM_API_KEY', 'LLM_BASE_URL', 'LLM_MODEL', 'GH_TOKEN']) {
    if (!process.env[name]?.trim()) throw new Error(`Missing required environment variable: ${name}`);
  }
  const baseURL = new URL(process.env.LLM_BASE_URL);
  if (baseURL.protocol !== 'https:' || baseURL.username || baseURL.password || baseURL.search || baseURL.hash) {
    throw new Error('LLM_BASE_URL must be an HTTPS API root without credentials, query, or fragment.');
  }
  const cache = new Cache(process.env.EXPLAINER_CACHE_DIR ?? path.join(directory, '.cache'));
  const prompt = await readFile(path.join(directory, 'prompt.md'), 'utf8');
  const architecture = await readFile(architectureFile, 'utf8');
  const key = hash({
    repo, number, base, head, title: pr.title, description: pr.body,
    model: process.env.LLM_MODEL, endpoint: baseURL.href,
    implementation: await implementationDigest(), architecture, tools,
  });
  const publisher = new Publisher({ api, repo, number, base, head, metadata: { title: pr.title, body: pr.body },
    cache, key, validate: validateExplanation });
  const existing = await publisher.existing();
  if (existing?.body.includes(`<!-- analysis:${key} -->`)) {
    console.log('Explanation already covers these inputs; no LLM request needed.');
    return;
  }
  const cached = await cache.get(['explanation', key]);
  if (cached) {
    await publisher.publish(cached, { cached: true });
    console.log('Restored the completed explanation without calling the LLM.');
    return;
  }
  const compareEndpoint = `repos/${repo}/compare/${base}...${head}`;
  const [comparison, diff] = await Promise.all([
    cache.remember(['comparison', repo, base, head], () => api(compareEndpoint)),
    cache.remember(['diff', repo, base, head], () => api(compareEndpoint, { raw: true })),
  ]);
  const files = comparison.files ?? [];
  const mergeBase = comparison.merge_base_commit?.sha;
  if (mergeBase !== base && !/^[a-f0-9]{40}$/.test(mergeBase ?? '')) throw new Error('Comparison has no valid merge base.');
  const handlers = {
    get_pr_diff: async () => clip(diff, 60000),
    get_file_diff: async ({ path: file }) => {
      const changed = files.find(entry => entry.filename === file);
      if (!changed) throw new Error('File not present in the comparison (GitHub limits this manifest to 300 files).');
      return changed.patch ? clip(changed.patch, 24000) : { error: 'Patch unavailable (binary, large, or omitted); read the file at base/head.' };
    },
    read_file: async ({ path: file, revision, start_line: start, end_line: end }) => {
      const sha = revision === 'base' ? base : head;
      const content = await cache.remember(['file', repo, sha, file], async () => {
        const encoded = file.split('/').map(encodeURIComponent).join('/');
        const result = await api(`repos/${repo}/contents/${encoded}?ref=${sha}`);
        if (result.type !== 'file' || result.encoding !== 'base64' || result.size > 256000) {
          throw new Error('File is unavailable, not a regular file, or exceeds the 256 KB limit.');
        }
        const text = Buffer.from(result.content, 'base64').toString('utf8');
        if (text.includes('\0')) throw new Error('Binary files cannot be read as code.');
        return text;
      });
      const lines = content.split('\n');
      return { path: file, sha, totalLines: lines.length, ...clip(lines.slice(start - 1, end)
        .map((line, i) => `${start + i}: ${line}`).join('\n'), 18000) };
    },
    get_pr_checks: async () => {
      const [pages, status] = await Promise.all([
        api(`repos/${repo}/commits/${head}/check-runs?per_page=100`, { paginate: true }),
        api(`repos/${repo}/commits/${head}/status`),
      ]);
      publisher.checksRead = true;
      const runs = pages.flatMap(page => page.check_runs);
      return {
        check_runs: runs.slice(0, 100).map(run => ({ name: run.name, status: run.status, conclusion: run.conclusion, url: run.html_url })),
        combined_status: status.state,
        statuses: status.statuses.slice(0, 100).map(item => ({ context: item.context, state: item.state })),
        truncated: runs.length > 100 || status.statuses.length > 100,
      };
    },
    publish_explanation: ({ body }) => publisher.publish(body),
  };
  const messages = [
    // Stable prefix helps providers that automatically cache repeated prompt prefixes.
    { role: 'system', content: `${prompt}\n\nProject architecture (reference evidence):\n${architecture}` },
    { role: 'user', content: JSON.stringify({
      repository: repo, number, title: pr.title, description: clip(pr.body ?? '', 8000),
      base, head, mergeBase, fileLinkPrefix: `https://github.com/${repo}/blob/`,
      changedFiles: pr.changed_files,
      manifestTruncated: pr.changed_files > files.length,
      files: files.map(file => ({ path: file.filename, previousPath: file.previous_filename, status: file.status, additions: file.additions, deletions: file.deletions })),
      diff: clip(diff, 60000),
    }) },
  ];
  const client = createClient({ apiKey: process.env.LLM_API_KEY, baseURL: baseURL.href, timeout: 90000, maxRetries: 1 });
  await runAgent({ client, model: process.env.LLM_MODEL, messages, handlers, onProgress: message => console.log(message) });
  console.log('PR explanation published.');
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch(error => {
    // API errors can include request bodies, headers, or sensitive provider responses.
    if (error instanceof OpenAI.APIError || error instanceof OpenAI.APIConnectionError) {
      console.error(`LLM request failed${error.status ? ` (HTTP ${error.status})` : ''}. Check endpoint/model configuration and provider availability.`);
    } else {
      console.error(error.message);
    }
    process.exitCode = 1;
  });
}
