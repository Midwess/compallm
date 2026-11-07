import type { PolicyJson, EntityUidJson } from '@cedar-policy/cedar-wasm/web'
import type { PolicyProjection } from '../../../src/pages/settings'
import { policyActions } from './fixtures'

let engine: Promise<typeof import('@cedar-policy/cedar-wasm/web')> | undefined
export function cedar() {
  engine ??= import('@cedar-policy/cedar-wasm/web').then(async (c) => {
    await c.default()
    return c
  })
  return engine
}
function referencesResource(expression: unknown): boolean {
  if (!expression || typeof expression !== 'object') return false
  if ('Var' in expression && expression.Var === 'resource') return true
  return Object.values(expression).some(referencesResource)
}
function entity(value: EntityUidJson) {
  return '__entity' in value ? value.__entity : value
}
function actionId(json: PolicyJson) {
  return json.action.op === '==' && 'entity' in json.action ? entity(json.action.entity).id : ''
}
export async function project(source: string): Promise<{ projection: PolicyProjection[]; diagnostics: string[] }> {
  const c = await cedar(),
    parsed = c.policySetTextToParts(source)
  if (parsed.type === 'failure') return { projection: [], diagnostics: parsed.errors.map((e) => e.message) }
  const projection: PolicyProjection[] = []
  for (const statement of parsed.policies) {
    const p = c.policyToJson(statement)
    if (p.type === 'failure') {
      projection.push({ action: '', source: statement, advanced: true, editable: false })
      continue
    }
    const json = p.json,
      action = actionId(json)
    const recognized =
      json.effect === 'permit' &&
      json.principal.op === 'All' &&
      json.resource.op === 'in' &&
      'entity' in json.resource &&
      entity(json.resource.entity).type === 'Compallm::Organization' &&
      entity(json.resource.entity).id === 'org-7' &&
      json.action.op === '==' &&
      'entity' in json.action &&
      entity(json.action.entity).type === 'Compallm::Action' &&
      policyActions.some((p) => p.id === action)
    const own =
      action === 'usage.read.own' &&
      json.conditions.length === 1 &&
      json.conditions[0].kind === 'when' &&
      JSON.stringify(json.conditions[0].body) ===
        JSON.stringify({
          '==': {
            left: { '.': { left: { Var: 'resource' }, attr: 'owner' } },
            right: { Var: 'principal' },
          },
        })
    const simple = recognized && !Object.keys(json.annotations ?? {}).length && (own || json.conditions.length === 0)
    const editable = simple && source.includes(statement) && !statement.includes('//') && !statement.includes('/*')
    projection.push({
      action: recognized ? action : '',
      source: statement,
      resourceRestricted: !simple && json.conditions.some((condition) => referencesResource(condition.body)),
      advanced: !editable,
      editable,
    })
  }
  for (const template of parsed.policy_templates)
    projection.push({ action: '', source: template, advanced: true, editable: false })
  return { projection, diagnostics: [] }
}
