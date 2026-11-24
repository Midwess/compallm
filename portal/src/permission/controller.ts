import type { AuthUser } from '@compallm/storybook/pages/auth'
import { PermissionMeStore } from '@compallm/contract/houdini'
import {
  createPermissionStore,
  permissionKey,
  publishPermissions,
  type PermissionController as Contract,
  type PermissionCheck,
  type PermissionFacts,
} from '@compallm/storybook'
import { ApiError, graphqlClient, requireData } from '../graphql-client'

import { cedarWorker } from '../workers/cedar-client'
import type { CedarOperations } from '../workers/cedar-protocol'

type Authority = CedarOperations['evaluatePermissions']['args'][0]
export class PermissionController implements Contract {
  readonly store = createPermissionStore()
  private scope = new AbortController()
  private authority?: Promise<Authority>
  private pending = new Map<string, Promise<void>>()
  private facts = new Map<string, PermissionFacts>()
  private batch?: { checks: Map<string, PermissionCheck>; operation: Promise<void>; generation: number }
  private generation = 0
  private active = true
  private identity?: Pick<AuthUser, 'id' | 'userId' | 'organizationId'>
  matches(user: AuthUser) {
    return (
      this.active &&
      this.identity?.id === user.id &&
      this.identity.userId === user.userId &&
      this.identity.organizationId === user.organizationId
    )
  }
  evaluate = async (checks: PermissionCheck[]) => {
    if (!this.active) return
    const scope = this.scope
    const generation = this.generation
    const waits: Promise<void>[] = []
    for (const check of checks) {
      const key = permissionKey(check)
      const pending = this.pending.get(key)
      if (pending) {
        waits.push(pending)
        continue
      }
      if (this.store.getState().entries[key]) continue
      const operation = this.enqueue(check)
      this.pending.set(key, operation)
      waits.push(operation)
    }
    await Promise.all(waits)
    if (this.active && this.scope === scope && !scope.signal.aborted && generation !== this.generation)
      await this.evaluate(checks)
  }
  private enqueue(check: PermissionCheck) {
    const generation = this.generation
    if (this.batch?.generation === generation) {
      this.batch.checks.set(permissionKey(check), check)
      return this.batch.operation
    }
    const scope = this.scope
    const checks = new Map([[permissionKey(check), check]])
    const operation = Promise.resolve()
      .then(async () => {
        if (this.batch?.operation === operation) this.batch = undefined
        const requested = [...checks.values()]
        let results: CedarOperations['evaluatePermissions']['result']
        try {
          if (!this.authority) throw new Error('Permissions are unavailable.')
          const verified = await this.authority
          if (scope.signal.aborted || !this.active) return
          results = await cedarWorker.request(
            'evaluatePermissions',
            [verified, requested, [...this.facts.values()]],
            scope.signal,
          )
        } catch {
          results = requested.map(() => 'error')
        }
        if (scope.signal.aborted || !this.active || this.scope !== scope) return
        const entries = Object.fromEntries(
          requested.flatMap((check, index) =>
            generation === this.generation || check.resourceType === 'Organization'
              ? [[permissionKey(check), { check, status: results[index] ?? 'error' }]]
              : [],
          ),
        )
        if (Object.keys(entries).length) this.store.setState((state) => ({ entries: { ...state.entries, ...entries } }))
      })
      .finally(() => {
        for (const key of checks.keys()) if (this.pending.get(key) === operation) this.pending.delete(key)
      })
    this.batch = { checks, operation, generation }
    return operation
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
      this.batch = undefined
      this.pending.clear()
      publishPermissions(
        this,
        Object.fromEntries(
          Object.entries(this.store.getState().entries).filter(([, entry]) => entry.check.resourceType === 'Organization'),
        ),
      )
    }
  }
  seed(user: AuthUser, node: import('@compallm/contract/houdini').PortalAuthority$result['me']['node']) {
    this.identity = user
    this.active = true
    this.seeded = {
      userId: node.id,
      organizationId: node.organizationId,
      source: node.cedarSource,
      entities: node.cedarEntities,
    }
    this.authority = Promise.resolve(this.seeded)
    this.store.setState({
      entries: Object.fromEntries(
        node.organizationPermissions.map(({ action, allowed }) => {
          const check = { action, resourceType: 'Organization' as const }
          return [permissionKey(check), { check, status: allowed ? ('allowed' as const) : ('denied' as const) }]
        }),
      ),
    })
  }
  private seeded?: Authority
  start = (user?: AuthUser) => {
    if (user && this.identity && !this.matches(user)) this.stop()
    this.identity = user
    this.active = true
    if (this.authority) return
    const scope = this.scope

    this.authority = (
      this.seeded
        ? Promise.resolve({
            me: {
              node: {
                id: this.seeded.userId,
                organizationId: this.seeded.organizationId,
                cedarSource: this.seeded.source,
                cedarEntities: this.seeded.entities,
              },
            },
          })
        : new PermissionMeStore()
            .fetch({ fetch: graphqlClient.fetch(scope.signal), policy: 'NoCache', blocking: true })
            .then(requireData)
    ).then(async ({ me: { node } }) => {
      const authority = {
        userId: node.id,
        organizationId: node.organizationId,
        source: node.cedarSource,
        entities: node.cedarEntities,
      }
      await cedarWorker.request('evaluatePermissions', [authority, [], []], scope.signal)
      return authority
    })
    void this.authority.catch(() => undefined)
  }

  stop = () => {
    this.identity = undefined
    this.seeded = undefined
    this.active = false
    this.scope.abort()
    this.scope = new AbortController()
    this.authority = undefined
    void cedarWorker.request('clearPermissions', []).catch(() => undefined)
    this.batch = undefined
    this.pending.clear()
    this.facts.clear()
    this.generation++
    publishPermissions(this, {})
  }
}
export async function requirePermissions(controller: Contract | undefined, checks: PermissionCheck[]) {
  if (!controller) return
  await controller.evaluate(checks)
  if (checks.some((check) => controller.store.getState().entries[permissionKey(check)]?.status !== 'allowed'))
    throw new ApiError('You do not have permission to perform this action.', 403, 'FORBIDDEN')
}
