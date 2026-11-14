import type { RoleUpdateInput } from '@compallm/contract/graphql'
import type { PolicyProjection } from '@compallm/storybook/pages/settings'

type Projection = { projection: PolicyProjection[]; diagnostics: string[] }
export interface CedarOperations {
  preparePolicySource: {
    args: [source: string, current: string, saved: string]
    result: { compact: string; length: number; matchesDraft: boolean; matchesSaved: boolean; input: RoleUpdateInput | null }
  }
  projectPolicy: { args: [source: string, catalog: [string, string][]]; result: Projection }
  formatPolicySource: { args: [source: string]; result: string }
  normalizePolicySource: { args: [source: string]; result: string }
  actionPolicies: { args: [source: string, action: string]; result: string[] }
  removePolicyAction: { args: [source: string, action: string]; result: string }
  editPolicyActions: {
    args: [source: string, actions: string[], enabled: boolean, catalog: [string, string][], retained: Record<string, string[]>]
    result: Projection & { source: string; disabledPolicies: Record<string, string[]> }
  }
}
export type CedarOperation = keyof CedarOperations
export type CedarResult<K extends CedarOperation> = CedarOperations[K]['result']
export type CedarRequest =
  | {
      [K in CedarOperation]: { type: 'request'; id: number; operation: K; args: CedarOperations[K]['args'] }
    }[CedarOperation]
  | { type: 'cancel'; id: number }
export type CedarResponse =
  | { type: 'ready' }
  | { type: 'fatal'; message: string }
  | { type: 'result'; id: number; result: CedarResult<CedarOperation> }
  | { type: 'error'; id: number; message: string }
