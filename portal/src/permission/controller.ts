import {
  createPermissionStore,
  createPermissionAbility,
  permissionKey,
  publishPermissions,
  type PermissionController as Contract,
  type PermissionCheck,
  type PermissionEntry,
} from '@compallm/storybook'
import { ApiError, graphql } from '../graphql-client'
import type { QueryGenqlSelection } from '@compallm/contract/graphql'

export class PermissionController implements Contract {
  readonly store = createPermissionStore()
  readonly ability = createPermissionAbility()
  private pending = new Map<string, { check: PermissionCheck; resolve(): void; promise: Promise<void> }>()
  private queued = new Set<string>()
  private scheduled = false
  private generation = 0
  private active = true
  private requests = new Set<AbortController>()
  ensure = async (checks: PermissionCheck[]) => {
    if (!this.active) return
    const waits: Promise<void>[] = []
    const entries = { ...this.store.getState().entries }
    for (const check of checks) {
      const key = permissionKey(check)
      const pending = this.pending.get(key)
      if (pending) {
        waits.push(pending.promise)
        continue
      }
      if (entries[key]) continue
      let resolve!: () => void
      const promise = new Promise<void>((complete) => {
        resolve = complete
      })
      this.pending.set(key, { check, resolve, promise })
      waits.push(promise)
      this.queued.add(key)
      entries[key] = { check, status: 'loading' }
    }
    this.store.setState({ entries })
    if (this.queued.size && !this.scheduled) {
      this.scheduled = true
      queueMicrotask(() => {
        this.scheduled = false
        void this.flush()
      })
    }
    await Promise.all(waits)
  }
  retry = async (checks: PermissionCheck[]) => {
    const entries = { ...this.store.getState().entries }
    for (const check of checks) {
      const key = permissionKey(check)
      if (entries[key]?.status === 'error') delete entries[key]
    }
    publishPermissions(this, entries)
    await this.ensure(checks)
  }
  private async flush() {
    const keys = [...this.queued]
    this.queued.clear()
    const generation = this.generation
    for (let offset = 0; offset < keys.length; offset += 100) {
      if (!this.active || generation !== this.generation) return
      const batch = keys.slice(offset, offset + 100)
      const pending = batch.map((key) => this.pending.get(key)!)
      const checks = pending.map((entry) => entry.check)
      const request = new AbortController()
      this.requests.add(request)
      try {
        const selection = {
          permissionDecisions: { __args: { checks }, edges: { node: { index: true, allowed: true } } },
        } satisfies QueryGenqlSelection
        const result = await graphql(request.signal).query(selection)
        if (!this.active || generation !== this.generation) return
        const nodes = result.permissionDecisions.edges.map(({ node }) => node)
        if (nodes.length !== batch.length || nodes.some((node, index) => node.index !== index))
          throw new Error('Incomplete permission decisions')
        const entries = { ...this.store.getState().entries }
        batch.forEach((key, index) => {
          entries[key] = { check: checks[index], status: nodes[index].allowed ? 'allowed' : 'denied' }
        })
        publishPermissions(this, entries)
      } catch {
        if (!this.active || generation !== this.generation) return
        const entries = { ...this.store.getState().entries }
        batch.forEach((key, index) => {
          entries[key] = { check: checks[index], status: 'error' } satisfies PermissionEntry
        })
        publishPermissions(this, entries)
      } finally {
        this.requests.delete(request)
        batch.forEach((key, index) => {
          pending[index].resolve()
          if (this.pending.get(key) === pending[index]) this.pending.delete(key)
        })
      }
    }
  }
  start = () => {
    this.active = true
    void this.ensure(
      (
        [
          'ROLE_READ',
          'ROLE_CREATE',
          'GROUP_READ',
          'GROUP_CREATE',
          'MEMBER_READ',
          'MEMBER_INVITE',
          'ORGANIZATION_SETTINGS_READ',
          'ORGANIZATION_SETTINGS_UPDATE',
          'ORGANIZATION_DELETE',
          'PROVIDER_READ',
          'PROVIDER_CREATE',
          'PROVIDER_MODEL_READ',
          'MODEL_READ',
          'MODEL_CREATE',
        ] as const
      ).map((action) => ({ action, resourceType: 'ORGANIZATION' })),
    )
  }
  stop = () => {
    this.active = false
    this.generation++
    for (const request of this.requests) request.abort()
    this.requests.clear()
    for (const pending of this.pending.values()) pending.resolve()
    this.pending.clear()
    this.queued.clear()
    publishPermissions(this, {})
  }
}

export async function requirePermissions(controller: Contract | undefined, checks: PermissionCheck[]) {
  if (!controller) return
  await controller.ensure(checks)
  if (
    checks.some(
      (check) =>
        controller.store.getState().entries[permissionKey(check)]?.status !== 'allowed' ||
        !controller.ability.can(check.action, permissionKey(check)),
    )
  ) {
    throw new ApiError('You do not have permission to perform this action.', 403, 'FORBIDDEN')
  }
}
