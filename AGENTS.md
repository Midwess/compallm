---
type: agent-instructions
scope: repository
---

# Agent workflow

Detect the user’s intent and follow the relevant flow. Read and apply the linked instructions and skills when their trigger applies; use multiple flows when requested. Explicit user instructions take precedence. Continue authorized work without requiring a PR or issue unless requested.

```xml
<flows>
  <flow request="discuss, research" steps="inspect → research → explain">
    <instructions path=".agents/instructions/documentation.md" />
  </flow>
  <flow request="implement, refactor" steps="inspect → document when required → code → verify">
    <instructions path=".agents/instructions/documentation.md" />
    <instructions path=".agents/instructions/engineering.md" />
    <instructions path=".agents/instructions/testing.md" />
  </flow>
  <flow request="write, edit, review prose" steps="inspect → write → review">
    <skill path=".agents/skills/writing/SKILL.md" />
    <instructions path=".agents/instructions/documentation.md" />
  </flow>
  <flow request="create issue" steps="clarify → write → create">
    <skill path=".agents/skills/create-issue/SKILL.md" />
    <skill when="bug" path=".agents/skills/create-bug/SKILL.md" />
    <skill when="feature" path=".agents/skills/create-feature/SKILL.md" />
    <skill when="task" path=".agents/skills/create-task/SKILL.md" />
  </flow>
  <flow request="find, link issue" steps="inspect → reuse or link">
    <skill path=".agents/skills/related-issues/SKILL.md" />
    <skill path=".agents/skills/link-issue/SKILL.md" />
  </flow>
  <flow request="create PR" steps="review → find issue → link → create">
    <instructions path=".agents/instructions/pull-request.md" />
  </flow>
</flows>
```

## Conditional instructions

- GraphQL schema, resolver, or client operation work → [GraphQL](.agents/instructions/graphql.md).
- Code comments, documentation, Wiki, or technical prose → [Writing](.agents/skills/writing/SKILL.md).
- Before planning or making changes → follow the Wiki and README discovery rules in [Documentation](.agents/instructions/documentation.md).
- New features or behavior/architecture changes → publish documentation first and record decisions as specified in [Documentation](.agents/instructions/documentation.md).
- Discussion or documentation only → do not implement the discussed changes.
- Preserve unrelated working-tree changes. Respect explicit instructions not to execute migrations; startup and database-backed tests can apply them automatically.
