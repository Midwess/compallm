import type { PolicyProjection } from '@compallm/storybook/pages/settings'
import type { EntityUidJson, PolicyJson } from '@cedar-policy/cedar-wasm/web'

let engine: Promise<typeof import('@cedar-policy/cedar-wasm/web')> | undefined

function cedar() {
  engine ??= import('@cedar-policy/cedar-wasm/web').then(async (module) => {
    await module.default()
    return module
  })
  return engine
}

export async function formatPolicySource(source: string) {
  const parser = await cedar()
  const normalized = await normalizePolicySource(source)
  const result = parser.formatPolicies({ policyText: normalized, lineWidth: 80, indentWidth: 2 })
  if (result.type === 'failure') throw new Error('Cedar formatting failed')
  return result.formatted_policy
}

function entity(value: EntityUidJson) {
  return '__entity' in value ? value.__entity : value
}

function actionEntities(policy: import('@cedar-policy/cedar-wasm/web').PolicyJson) {
  return policy.action.op === '==' && 'entity' in policy.action
    ? [policy.action.entity]
    : policy.action.op === 'in' && 'entities' in policy.action
      ? policy.action.entities
      : policy.action.op === 'in' && 'entity' in policy.action
        ? [policy.action.entity]
        : []
}

function sourceStatements(source: string) {
  const statements: { start: number; end: number; text: string; hasComment: boolean; comments: string[] }[] = []
  let start = -1
  let depth = 0
  let hasComment = false
  let comments: string[] = []
  for (let index = 0; index < source.length; index++) {
    const char = source[index]
    if (char === '/' && source[index + 1] === '/') {
      if (start >= 0) hasComment = true
      const commentStart = index
      while (index < source.length && source[index] !== '\n') index++
      if (start >= 0) comments.push(source.slice(commentStart, index))
      continue
    }
    if (char === '/' && source[index + 1] === '*') {
      if (start >= 0) hasComment = true
      const commentStart = index
      index += 2
      while (index < source.length && !(source[index] === '*' && source[index + 1] === '/')) index++
      index++
      if (start >= 0) comments.push(source.slice(commentStart, index + 1))
      continue
    }
    if (start < 0) {
      if (/\s/u.test(char)) continue
      start = index
      hasComment = false
      comments = []
    }
    if (char === '"') {
      for (index++; index < source.length; index++) {
        if (source[index] === '\\') index++
        else if (source[index] === '"') break
      }
      continue
    }
    if ('([{'.includes(char)) depth++
    else if (')]}'.includes(char)) depth--
    else if (char === ';' && depth === 0) {
      statements.push({ start, end: index + 1, text: source.slice(start, index + 1), hasComment, comments })
      start = -1
    }
  }
  return statements
}

function stableJson(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(stableJson).join(',')}]`
  if (value && typeof value === 'object')
    return `{${Object.entries(value)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([key, entry]) => `${JSON.stringify(key)}:${stableJson(entry)}`)
      .join(',')}}`
  return JSON.stringify(value) ?? 'null'
}

function policyGroup(policy: PolicyJson) {
  return stableJson({
    effect: policy.effect,
    principal: policy.principal,
    resource: policy.resource,
    conditions: policy.conditions,
    annotations: policy.annotations ?? {},
  })
}

export async function normalizePolicySource(source: string) {
  const parser = await cedar()
  const parsed = parser.policySetTextToParts(source)
  if (parsed.type === 'failure') throw new Error('Invalid policy')
  const statements = sourceStatements(source)
  if (statements.length !== parsed.policies.length + parsed.policy_templates.length)
    throw new Error('Unable to locate policy statements')
  const groups = new Map<string, { policy: PolicyJson; actions: Map<string, EntityUidJson>; spans: typeof statements }>()
  for (const span of statements) {
    if (span.hasComment || !parsed.policies.includes(span.text)) continue
    const result = parser.policyToJson(span.text)
    if (result.type === 'failure' || result.json.action.op === 'All') continue
    const actions = actionEntities(result.json)
    if (!actions.length) continue
    const actionGroup = result.json.action.op === '==' ? `equals:${stableJson(entity(actions[0]))}` : 'in'
    const key = `${policyGroup(result.json)}:${actionGroup}`
    const group = groups.get(key) ?? { policy: result.json, actions: new Map<string, EntityUidJson>(), spans: [] }
    for (const value of actions) group.actions.set(stableJson(entity(value)), value)
    group.spans.push(span)
    groups.set(key, group)
  }
  const edits: { start: number; end: number; text: string }[] = []
  for (const group of groups.values()) {
    const actions = [...group.actions.entries()].sort(([left], [right]) => left.localeCompare(right)).map(([, value]) => value)
    const canonical: PolicyJson = {
      ...group.policy,
      action: group.policy.action.op === '==' ? { op: '==', entity: actions[0] } : { op: 'in', entities: actions },
    }
    const rendered = parser.policyToText(canonical)
    if (rendered.type === 'failure') throw new Error('Unable to normalize policy')
    const checked = parser.policyToJson(rendered.text)
    if (
      checked.type === 'failure' ||
      policyGroup(checked.json) !== policyGroup(group.policy) ||
      stableJson(actionEntities(checked.json).map(entity)) !== stableJson(actions.map(entity))
    )
      throw new Error('Policy normalization changed meaning')
    group.spans.forEach((span, index) =>
      edits.push({ start: span.start, end: span.end, text: index === 0 ? rendered.text : '' }),
    )
  }
  for (const edit of edits.sort((left, right) => right.start - left.start))
    source = source.slice(0, edit.start) + edit.text + source.slice(edit.end)
  if (parser.policySetTextToParts(source).type === 'failure') throw new Error('Invalid normalized policy')
  return source
}

export function actionStatement(action: string, resourceType: string) {
  const organization = 'Compallm::Organization::"current"'
  const principal = 'Compallm::Role::"current"'
  const resource =
    resourceType === 'Organization' ? `resource == ${organization}` : `resource is Compallm::${resourceType} in ${organization}`
  return `permit(principal in ${principal}, action == Compallm::Action::${JSON.stringify(action)}, ${resource});`
}

export async function projectPolicy(source: string, actionsByName: ReadonlyMap<string, string>) {
  const parser = await cedar()
  const parsed = parser.policySetTextToParts(source)
  if (parsed.type === 'failure')
    return {
      projection: [] as PolicyProjection[],
      diagnostics: parsed.errors.map((error) => error.message),
    }
  const projection: PolicyProjection[] = []
  for (const statement of parsed.policies) {
    const result = parser.policyToJson(statement)
    if (result.type === 'failure') {
      projection.push({ action: '', source: statement, advanced: true, editable: false })
      continue
    }
    const policy = result.json
    const actions = actionEntities(policy)
    const names =
      policy.action.op === 'All'
        ? [...actionsByName.keys()]
        : actions
            .map(entity)
            .filter((action) => action.type === 'Compallm::Action' && actionsByName.has(action.id))
            .map((action) => action.id)
    const unknownActions = actions
      .map(entity)
      .filter((action) => action.type !== 'Compallm::Action' || !actionsByName.has(action.id))
    if (!names.length || unknownActions.length)
      projection.push({
        action: '',
        effect: policy.effect,
        source: statement,
        advanced: true,
        editable: false,
        reason: unknownActions.length
          ? 'Contains actions outside the current catalog. Edit them in Cedar.'
          : 'Edit this statement in Cedar.',
      })
    for (const action of names) {
      const editable =
        policy.effect === 'permit' &&
        policy.action.op !== 'All' &&
        actions.length > 0 &&
        sourceStatements(source).some((span) => span.text === statement)
      projection.push({
        action,
        effect: policy.effect,
        source: statement,
        advanced: !editable,
        editable,
        resourceRestricted: policy.conditions.length > 0,
        reason: policy.action.op === 'All' ? 'Applies to all actions. Edit its action scope in Cedar.' : undefined,
      })
    }
  }
  for (const statement of parsed.policy_templates) {
    projection.push({
      action: '',
      source: statement,
      advanced: true,
      editable: false,
      reason: 'This is a policy template. Edit it in Cedar.',
    })
  }
  return { projection, diagnostics: [] as string[] }
}

export async function actionPolicies(source: string, action: string) {
  const parser = await cedar()
  const parsed = parser.policySetTextToParts(source)
  if (parsed.type === 'failure') throw new Error('Invalid policy')
  const policies: string[] = []
  for (const statement of parsed.policies) {
    const result = parser.policyToJson(statement)
    if (result.type === 'failure' || result.json.effect !== 'permit') continue
    const matching = actionEntities(result.json).filter(
      (value) => entity(value).type === 'Compallm::Action' && entity(value).id === action,
    )
    if (!matching.length) continue
    const policy: PolicyJson = {
      ...result.json,
      action: result.json.action.op === '==' ? { op: '==', entity: matching[0] } : { op: 'in', entities: matching },
    }
    const rendered = parser.policyToText(policy)
    if (rendered.type === 'failure') throw new Error('Unable to retain action policy')
    policies.push(rendered.text)
  }
  return [...new Set(policies)]
}

export async function removePolicyAction(source: string, action: string) {
  const parser = await cedar()
  const parsed = parser.policySetTextToParts(source)
  if (parsed.type === 'failure') throw new Error('Invalid policy')
  for (const span of sourceStatements(source).reverse()) {
    if (!parsed.policies.includes(span.text)) continue
    const result = parser.policyToJson(span.text)
    if (result.type === 'failure' || result.json.effect !== 'permit') continue
    const values = actionEntities(result.json)
    const remaining = values.filter((value) => entity(value).type !== 'Compallm::Action' || entity(value).id !== action)
    if (values.length === remaining.length) continue
    let replacement = ''
    if (remaining.length) {
      const policy: PolicyJson = { ...result.json, action: { op: 'in', entities: remaining } }
      const rendered = parser.policyToText(policy)
      if (rendered.type === 'failure') throw new Error('Unable to update action list')
      const checked = parser.policyToJson(rendered.text)
      if (
        checked.type === 'failure' ||
        policyGroup(checked.json) !== policyGroup(policy) ||
        checked.json.action.op !== policy.action.op ||
        stableJson(actionEntities(checked.json).map(entity)) !== stableJson(actionEntities(policy).map(entity))
      )
        throw new Error('Policy edit changed other rules')
      replacement = rendered.text
    }
    const comments = span.comments.length ? span.comments.join('\n') + '\n' : ''
    source = source.slice(0, span.start) + comments + replacement + source.slice(span.end)
  }
  if (parser.policySetTextToParts(source).type === 'failure') throw new Error('Invalid updated policy')
  return source
}
