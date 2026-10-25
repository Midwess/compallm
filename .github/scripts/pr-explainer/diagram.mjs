import { JSDOM } from 'jsdom';

// Parse only: no Chromium download/startup or rendering needed in CI.
const dom = new JSDOM('');
globalThis.window = dom.window;
globalThis.document = dom.window.document;
const { default: mermaid } = await import('mermaid');
delete globalThis.window;
delete globalThis.document;
mermaid.initialize({ startOnLoad: false, securityLevel: 'strict' });

export async function validateExplanation(body) {
  if (typeof body !== 'string' || body.length < 100 || body.length > 16000) {
    throw new Error('Comment must contain 100–16000 characters.');
  }
  for (const heading of ['Idea', 'What changes', 'Review focus', 'Checks / coverage']) {
    if (!body.includes(`## ${heading}\n`)) throw new Error(`Missing heading: ## ${heading}`);
  }
  const blocks = [...body.matchAll(/^```mermaid\s*\n([\s\S]*?)^```\s*$/gm)];
  const openings = [...body.matchAll(/^```mermaid\b/gm)];
  if (blocks.length !== openings.length || blocks.length > 1) {
    throw new Error('Use at most one correctly closed Mermaid block.');
  }
  const prose = body.replace(/^```mermaid\s*\n[\s\S]*?^```\s*$/gm, '')
    .replace(/\[([^\]]+)\]\([^)]*\)/g, '$1');
  const words = prose.trim().split(/\s+/).length;
  const visible = prose.replace(/`[^`\n]*`/g, '');
  const errors = [];
  if (words > 350) errors.push(`Explanation has ${words} words. Use at most 350 outside the diagram; shorten the bullets and coverage notes.`);
  if (/<!--/.test(body)) errors.push('Remove hidden HTML comments.');
  if (/<\/?[a-z][^>]*>/i.test(visible)) errors.push('Remove HTML tags; put code types in backticks.');
  if (/(^|[\s(])@[\w-]+/m.test(visible)) errors.push('Remove GitHub mentions. Email addresses and inline code are allowed.');
  if (/!\[/.test(visible)) errors.push('Remove embedded images.');
  if (errors.length) throw new Error(errors.join(' '));
  for (const [, source] of blocks) {
    if (!/^\s*(flowchart\s+(TD|TB|LR|RL|BT)|sequenceDiagram)\b/.test(source)
      || /%%|\bclick\b|https?:|\binit\b|^\s*---/im.test(source)
      || source.length > 4000) {
      throw new Error('Use a small flowchart or sequence diagram without directives or external links.');
    }
    try {
      await mermaid.parse(source);
    } catch {
      throw new Error('Invalid Mermaid syntax. Simplify the diagram and correct its syntax.');
    }
  }
  return body.trim();
}
