import type { PolicyProjection } from '@compallm/storybook/pages/settings'
import type { EntityUidJson } from '@cedar-policy/cedar-wasm/web'

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
  const result = parser.formatPolicies({ policyText: source, lineWidth: 80, indentWidth: 2 })
  if (result.type === 'failure') throw new Error('Cedar formatting failed')
  return result.formatted_policy
}

function entity(value: EntityUidJson) {
  return '__entity' in value ? value.__entity : value
}

export function actionStatement(action: string, organizationId: string, roleId: string, resourceType: string) {
  const organization = `Compallm::Organization::${JSON.stringify(organizationId)}`
  const principal = `Compallm::Role::${JSON.stringify(`${organizationId}:${roleId}`)}`
  const resource =
    resourceType === 'Organization' ? `resource == ${organization}` : `resource is Compallm::${resourceType} in ${organization}`
  return `permit(principal in ${principal}, action == Compallm::Action::${JSON.stringify(action)}, ${resource});`
}

export async function projectPolicy(
  source: string,
  actionsByName: ReadonlyMap<string, string>,
  organizationId: string,
  roleId: string,
) {
  const parser = await cedar()
  const parsed = parser.policySetTextToParts(source)
  if (parsed.type === 'failure')
    return { projection: [] as PolicyProjection[], diagnostics: parsed.errors.map((error) => error.message) }
  const projection: PolicyProjection[] = []
  for (const statement of parsed.policies) {
    const result = parser.policyToJson(statement)
    if (result.type === 'failure') {
      projection.push({ action: '', source: statement, advanced: true, editable: false })
      continue
    }
    const policy = result.json
    const actions =
      policy.action.op === '==' && 'entity' in policy.action
        ? [policy.action.entity]
        : policy.action.op === 'in' && 'entities' in policy.action
          ? policy.action.entities
          : policy.action.op === 'in' && 'entity' in policy.action
            ? [policy.action.entity]
            : []
    const names =
      policy.effect === 'permit'
        ? actions
            .map(entity)
            .filter((action) => action.type === 'Compallm::Action' && actionsByName.has(action.id))
            .map((action) => action.id)
        : []
    if (!names.length) projection.push({ action: '', source: statement, advanced: true, editable: false })
    for (const action of names) {
      const position = source.indexOf(statement)
      const editable =
        statement === actionStatement(action, organizationId, roleId, actionsByName.get(action)!) &&
        position >= 0 &&
        position === source.lastIndexOf(statement)
      projection.push({ action, source: statement, advanced: !editable, editable, resourceRestricted: !editable })
    }
  }
  for (const statement of parsed.policy_templates)
    projection.push({ action: '', source: statement, advanced: true, editable: false })
  return { projection, diagnostics: [] as string[] }
}
