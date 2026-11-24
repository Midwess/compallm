import { createStore, type StateStore as StoreApi } from './state.svelte'
import type { SchemaCacheTypes } from '@compallm/contract/houdini'
type PolicyAction = { name: SchemaCacheTypes['types']['PolicyAction']['fields']['name']['type'] }
export type PermissionAction = PolicyAction['name']
export type PermissionResourceType = 'Organization' | 'Role' | 'Group' | 'Member' | 'Provider' | 'ProviderModel' | 'Model'

export interface PermissionCheck {
  action: PermissionAction
  resourceType: PermissionResourceType
  resourceId?: string | null
  roleId?: string | null
  memberId?: string | null
  providerModelId?: string | null
}
export interface PermissionFacts {
  resourceType: PermissionResourceType
  id: string
  enabled?: boolean
  providerId?: string
  roleIds?: string[]
  groupIds?: string[]
}
export type PermissionStatus = 'allowed' | 'denied' | 'error'
export interface PermissionEntry {
  check: PermissionCheck
  status: PermissionStatus
}
export interface PermissionState {
  entries: Record<string, PermissionEntry>
}
export interface PermissionController {
  readonly store: StoreApi<PermissionState>
  setFacts?(facts: PermissionFacts[]): void
  evaluate(checks: PermissionCheck[]): Promise<void>
}
export interface PermissionRequestInput {
  enabled?: boolean
  actions: readonly PermissionAction[]
  resource?: { type: PermissionResourceType; id?: string | null }
  context?: { roleId?: string; memberId?: string; providerModelId?: string }
  match?: 'all' | 'any'
}
export function permissionChecks(request: PermissionRequestInput): PermissionCheck[] {
  if (request.enabled === false) return []
  return request.actions.map((action) => ({
    action,
    resourceType: request.resource?.type ?? 'Organization',
    resourceId: request.resource?.id,
    roleId: request.context?.roleId,
    memberId: request.context?.memberId,
    providerModelId: request.context?.providerModelId,
  }))
}
export function permissionKey(check: PermissionCheck): string {
  return JSON.stringify([
    check.action,
    check.resourceType,
    check.resourceId ?? null,
    check.roleId ?? null,
    check.memberId ?? null,
    check.providerModelId ?? null,
  ])
}
export function createPermissionStore() {
  return createStore<PermissionState>(() => ({ entries: {} }))
}
export function publishPermissions(controller: Pick<PermissionController, 'store'>, entries: PermissionState['entries']) {
  controller.store.setState({ entries })
}
