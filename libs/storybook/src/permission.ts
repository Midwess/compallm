import { Ability } from '@casl/ability'
import { createStore, type StoreApi } from 'zustand/vanilla'
import type { PermissionAction, PermissionCheckInput, PermissionResourceType } from '@compallm/contract/graphql'

export type PermissionCheck = PermissionCheckInput
export type PermissionStatus = 'loading' | 'allowed' | 'denied' | 'error'
export interface PermissionEntry {
  check: PermissionCheck
  status: PermissionStatus
}
export interface PermissionState {
  entries: Record<string, PermissionEntry>
}
export type PermissionAbility = Ability<[PermissionAction, string]>
export interface PermissionController {
  readonly store: StoreApi<PermissionState>
  readonly ability: PermissionAbility
  ensure(checks: PermissionCheck[]): Promise<void>
  retry(checks: PermissionCheck[]): Promise<void>
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
    resourceType: request.resource?.type ?? 'ORGANIZATION',
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
export function createPermissionAbility(): PermissionAbility {
  return new Ability<[PermissionAction, string]>()
}
export function publishPermissions(
  controller: Pick<PermissionController, 'store' | 'ability'>,
  entries: PermissionState['entries'],
) {
  controller.ability.update(
    Object.entries(entries)
      .filter(([, entry]) => entry.status === 'allowed')
      .map(([key, entry]) => ({ action: entry.check.action, subject: key })),
  )
  controller.store.setState({ entries })
}
export { AbilityProvider as PermissionAbilityProvider } from '@casl/react'
