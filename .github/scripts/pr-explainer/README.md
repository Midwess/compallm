# AI PR explanation

This GitHub Actions runner explains the idea of a PR, describes meaningful behavior
changes, and adds a small Mermaid diagram when useful. It can flag up to three
supported review concerns; it does not approve PRs or request changes.

## Configuration

Set these in repository **Settings → Secrets and variables → Actions**:

| Name | Kind | Value |
| --- | --- | --- |
| `LLM_API_KEY` | Secret | API credential for your configured provider |
| `LLM_BASE_URL` | Secret | HTTPS API root, including any `/v1` prefix; exclude `/chat/completions` |
| `LLM_MODEL` | Variable | Model identifier accepted by that endpoint |

The endpoint must implement OpenAI-compatible Chat Completions with function tools.
There are no provider names, model defaults, or provider-specific request options in
the runner. `GH_TOKEN` comes from GitHub's job-scoped token automatically; no PAT is
required. Repository/organization settings must permit the workflow's PR write access.

The workflow runs for non-draft, same-repository PRs into `main` on opening, new
commits, reopening, becoming ready, and metadata edits. Fork PRs and bot-authored PRs
are skipped. After this implementation is merged into the base branch, opening or
updating an eligible PR starts it. This initial implementation PR cannot run its
new runner until the runner exists on the trusted base branch.

After merge, an existing PR can be explained manually using:

```bash
gh workflow run pr-explainer.yml --ref main -f pr_number=11
```

Manual runs also read only same-repository, open, non-draft PRs. They load runner
code from the default branch, never from the PR being analyzed.

## Execution and permissions

The workflow uses `pull_request_target`, checking out only the runner and architecture
document from the event's exact base SHA. It never checks out, installs dependencies
from, or executes the PR head. Keep that restriction when editing the workflow.
Proposed code is retrieved as data using the installed `gh` CLI.

The OpenAI JavaScript SDK connects using the configured environment. The script
passes function definitions, executes requested functions, appends the tool results
with their matching call IDs, and continues until `publish_explanation` succeeds.
This follows the [official function-calling flow](https://developers.openai.com/api/docs/guides/function-calling).

Tools can read the pinned diff, individual patches, base/head file ranges, and check
results. They cannot run arbitrary commands or choose another repository/PR. `gh`
receives argument arrays and a restricted environment without the LLM credentials.
PR data is untrusted evidence; the prompt explicitly rejects instructions found in it.
Code and PR context are sent to the configured LLM provider; API credentials are not
included in prompts, cache entries, or tool results.

The publisher requires a check-status read, validates Markdown and Mermaid syntax,
then rechecks the PR's base/head, title/body, and readiness before writing. It updates the
`github-actions[bot]` comment beginning with `<!-- ai-pr-explanation -->`, or creates
that comment if none exists. Human comments are never modified. The footer identifies
the analyzed commit and notes that checks reflect analysis time. A head update just
after the final GitHub read can still race with publication; the footer and subsequent
PR run make that visible and replace the old explanation.

## Performance and limits

- npm downloads and installed dependencies are cached using the committed lockfile.
- Immutable comparisons, diffs, and files are cached by repository and commit SHA,
  with duplicate reads coalesced within a run. Check results remain live.
- Completed, published explanations are keyed by PR title/body, base/head SHAs,
  model, endpoint hash, runner/dependency/prompt version, tools, and architecture.
  Matching comments skip LLM calls. A cached explanation can restore a deleted comment.
- Per-PR data caches are saved under unique run keys, restoring the newest prefix
  match. Old caches are subject to GitHub cache retention/eviction. Cache contents
  include code and generated explanations, and follow GitHub's repository cache
  access rules. Never add credentials or raw model conversations to them.
- Instructions and architecture form a stable prompt prefix. Provider-side prompt
  caching is opportunistic; no provider-specific cache parameters are sent.
- Superseded runs are canceled per PR. Model requests time out after 90 seconds
  with at most one SDK retry; `gh` requests time out after 45 seconds. The job has a
  15-minute ceiling, ten model rounds, forty tool calls, and a 240,000-character
  conversation budget. These are character limits, not a token guarantee for every model.
  The final three rounds are reserved for fresh checks, publication, and a validation
  repair, so code exploration cannot consume the entire conversation budget.
- Initial diffs are capped at 60,000 characters, individual patches at 24,000, files
  at 256 KB, and reads at 300 lines/18,000 characters. Truncation is explicit. GitHub's
  comparison manifest covers at most 300 files; the prompt requires disclosure of
  incomplete coverage. If the full diff exceeds the CLI's 8 MB output limit, the run
  fails rather than publishing an unsupported explanation.

Mermaid syntax is parsed locally using Mermaid and JSDOM without launching Chromium,
which avoids browser downloads and startup latency. No diagram images are uploaded;
GitHub renders the fenced Mermaid source in the comment.

## Development

From this directory:

```bash
npm ci --ignore-scripts
npm test
```

Tests use mocked GitHub operations and the real SDK with a mocked HTTP transport.
They make no paid model calls and publish no comments. Tests cover cache reuse,
function-call protocol, unsafe arguments, Mermaid validation, stale PR protection,
and comment ownership. Do not run `npm start` locally unless you intend to publish
to the PR identified by `GITHUB_EVENT_PATH` and `GITHUB_REPOSITORY`.
