import {
  createPermissionStore,
  permissionKey,
  publishPermissions,
  type PermissionController as Contract,
  type PermissionCheck,
  type PermissionFacts,
} from '@compallm/storybook'
import { ApiError, graphql } from '../graphql-client'
import type { QueryGenqlSelection } from '@compallm/contract/graphql'
import { cedarWorker } from '../workers/cedar-client'
import type { CedarOperations } from '../workers/cedar-protocol'

type Authority = CedarOperations['evaluatePermissions']['args'][0]
export class PermissionController implements Contract {
  readonly store = createPermissionStore()
  private scope = new AbortController()
  private authority?: Promise<Authority>
  private pending = new Map<string, Promise<void>>()
  private facts = new Map<string, PermissionFacts>()
  private generation = 0
  private active = true
  private load() {
    if (!this.authority) {
      const scope = this.scope
      const selection = {
        me: { node: { id: true, organizationId: true, cedarSource: true, cedarEntities: true } },
      } satisfies QueryGenqlSelection
      this.authority = graphql(scope.signal)
        .query(selection)
        .then(({ me: { node } }) => ({
          userId: node.id,
          organizationId: node.organizationId,
          source: node.cedarSource,
          entities: node.cedarEntities,
        }))
    }
    return this.authority
  }
  ensure = async (checks: PermissionCheck[]) => {
    if (!this.active) return
    const waits: Promise<void>[] = []
    for (const check of checks) {
      const key = permissionKey(check)
      const pending = this.pending.get(key)
      if (pending) {
        waits.push(pending)
        continue
      }
      if (this.store.getState().entries[key]) continue
      this.store.setState((state) => ({ entries: { ...state.entries, [key]: { check, status: 'loading' } } }))
      const scope = this.scope
      const generation = this.generation
      const operation = this.load()
        .then(async (authority) => {
          const [result] = await cedarWorker.request(
            'evaluatePermissions',
            [authority, [check], [...this.facts.values()]],
            scope.signal,
          )
          if (!scope.signal.aborted && this.active && generation === this.generation)
            this.store.setState((state) => ({ entries: { ...state.entries, [key]: { check, status: result } } }))
        })
        .catch(() => {
          if (!scope.signal.aborted && this.active && generation === this.generation)
            this.store.setState((state) => ({ entries: { ...state.entries, [key]: { check, status: 'error' } } }))
        })
        .finally(() => {
          if (this.pending.get(key) === operation) this.pending.delete(key)
        })
      this.pending.set(key, operation)
      waits.push(operation)
    }
    await Promise.all(waits)
  }
  retry = async (checks: PermissionCheck[]) => {
    this.authority = undefined
    const entries = { ...this.store.getState().entries }
    for (const check of checks) if (entries[permissionKey(check)]?.status === 'error') delete entries[permissionKey(check)]
    publishPermissions(this, entries)
    await this.ensure(checks)
  }
  setFacts = (facts: PermissionFacts[]) => {
    let changed = false
    for (const fact of facts) {
      const key = `${fact.resourceType}:${fact.id}`
      if (JSON.stringify(this.facts.get(key)) === JSON.stringify(fact)) continue
      this.facts.set(key, fact)
      changed = true
    }
    if (changed) {
      this.generation++
      this.pending.clear()
      publishPermissions(
        this,
        Object.fromEntries(
          Object.entries(this.store.getState().entries).filter(
            ([, entry]) => entry.check.resourceType === 'Organization' && entry.status !== 'loading',
          ),
        ),
      )
    }
  }
  start = () => {
    this.active = true
    void this.ensure([
      { action: 'role.read', resourceType: 'Organization' },
      { action: 'group.read', resourceType: 'Organization' },
      { action: 'model.read', resourceType: 'Organization' },
    ])
  }
  stop = () => {
    this.active = false
    this.scope.abort()
    this.scope = new AbortController()
    this.authority = undefined
    this.pending.clear()
    this.facts.clear()
    this.generation++
    publishPermissions(this, {})
  }
}
export async function requirePermissions(controller: Contract | undefined, checks: PermissionCheck[]) {
  if (!controller) return
  await controller.ensure(checks)
  if (checks.some((check) => controller.store.getState().entries[permissionKey(check)]?.status !== 'allowed'))
    throw new ApiError('You do not have permission to perform this action.', 403, 'FORBIDDEN')
}
