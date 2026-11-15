import { permissionKey, type PermissionController, type FormStore } from '@compallm/storybook'
import type { SettingsState } from '@compallm/storybook/pages/settings'
import type { QueryGenqlSelection } from '@compallm/contract/graphql'
import { graphql } from '../../graphql-client'
import { requirePermissions } from '../../permission/controller'
import { notify } from '../../notifications/toast'
const pageInfo = { endCursor: true, hasNextPage: true } as const
export class MembersController {
  private request?: AbortController
  private stop?: () => void
  private appliedQuery = ''
  private cursor?: string
  private history: (string | undefined)[] = []
  private lifecycle?: AbortController
  private readonly store: FormStore<SettingsState>
  private readonly permissions?: PermissionController
  constructor(store: FormStore<SettingsState>, permissions?: PermissionController) {
    this.store = store
    this.permissions = permissions
  }
  start = () => {
    this.cancel()
    this.lifecycle = new AbortController()
    this.appliedQuery = this.store.getState().query.trim()
    this.cursor = undefined
    this.history = []
    this.stop = this.store.subscribe((next, previous) => {
      if (next.memberOrder !== previous.memberOrder || next.filters.memberRole !== previous.filters.memberRole) {
        this.cursor = undefined
        this.history = []
        this.store.setState({
          membersHasNextPage: false,
          membersHasPreviousPage: false,
          membersEndCursor: null,
          membersPage: 1,
        })
        void this.load()
      }
    })
    void this.load()
    return this.cancel
  }
  cancel = () => {
    this.lifecycle?.abort()
    this.request?.abort()
    this.stop?.()
    this.stop = undefined
    this.lifecycle = undefined
  }
  search = (query: string) => {
    this.appliedQuery = query.trim()
    this.cursor = undefined
    this.history = []
    this.store.setState({
      query,
      membersHasNextPage: false,
      membersHasPreviousPage: false,
      membersEndCursor: null,
      membersPage: 1,
    })
    void this.load()
  }
  setOrder = (memberOrder: SettingsState['memberOrder']) => this.store.setState({ memberOrder })
  nextPage = () => {
    if (!this.store.getState().membersHasNextPage || this.store.getState().membersLoading) return
    const rollback = { cursor: this.cursor, history: [...this.history] }
    this.history.push(this.cursor)
    this.cursor = this.store.getState().membersEndCursor ?? undefined
    void this.load(rollback)
  }
  previousPage = () => {
    if (!this.history.length || this.store.getState().membersLoading) return
    const rollback = { cursor: this.cursor, history: [...this.history] }
    this.cursor = this.history.pop()
    void this.load(rollback)
  }
  private async load(rollback?: { cursor: string | undefined; history: (string | undefined)[] }) {
    const owner = this.lifecycle
    if (!owner) return
    this.request?.abort()
    const request = new AbortController()
    this.request = request
    this.store.setState({ membersLoading: true, membersComplete: false })
    try {
      if (this.permissions)
        await requirePermissions(this.permissions, [{ action: 'member.read', resourceType: 'Organization' }])
      if (request.signal.aborted || owner.signal.aborted) return
      const q = this.appliedQuery.replace(/[\\%_]/g, '\\$&')
      const role = this.store.getState().filters.memberRole
      const filter = {
        ...(q ? { or: [{ displayName: { ilike: `%${q}%` } }, { email: { ilike: `%${q}%` } }] } : {}),
        ...(role && role !== 'all' ? { roleId: { eq: role } } : {}),
      }
      const direction = this.store.getState().memberOrder === 'ASC_NULLS_LAST' ? 'AscNullsLast' : 'DescNullsLast'
      const relationshipChecks = (['group.read', 'role.read'] as const).map((action) => ({
        action,
        resourceType: 'Organization' as const,
      }))
      if (this.permissions) await this.permissions.evaluate(relationshipChecks)
      if (request.signal.aborted || owner.signal.aborted) return
      const includeRoles =
        !this.permissions ||
        this.permissions.store.getState().entries[permissionKey(relationshipChecks[1])]?.status === 'allowed'
      const includeGroups =
        !this.permissions ||
        this.permissions.store.getState().entries[permissionKey(relationshipChecks[0])]?.status === 'allowed'
      const selection = {
        me: { node: { id: true } },
        members: {
          __args: { first: 20, after: this.cursor, filter, currentUserFirst: true, orderBy: [{ displayName: direction }] },
          edges: {
            node: {
              id: true,
              displayName: true,
              email: true,
              available: true,
              joinedAt: true,
              ...(includeRoles ? { role: { id: true, name: true, description: true, color: true, owner: true } } : {}),
              ...(includeGroups ? { groups: { id: true, name: true, description: true } } : {}),
              ...(includeGroups && includeRoles ? { effectiveRoleIds: true } : {}),
            },
          },
          pageInfo,
        },
      } satisfies QueryGenqlSelection
      const result = await graphql(AbortSignal.any([request.signal, owner.signal])).query(selection)
      if (this.request !== request || request.signal.aborted || owner.signal.aborted) return
      this.permissions?.setFacts?.(
        result.members.edges.flatMap(({ node }) =>
          node.effectiveRoleIds && node.groups
            ? [
                {
                  resourceType: 'Member' as const,
                  id: node.id,
                  roleIds: node.effectiveRoleIds,
                  groupIds: node.groups.map((group) => group.id),
                },
              ]
            : [],
        ),
      )
      this.store.setState({
        currentMemberId: result.me?.node.id,
        members: result.members.edges.map(({ node }) => ({
          id: node.id,
          name: node.displayName || 'Member',
          email: node.email,
          role: node.role ?? null,
          groups: node.groups,
          available: node.available,
          joinedAt: node.joinedAt,
          limitUsd: null,
          currentUsageUsd: null,
          previousUsageUsd: null,
        })),
        membersComplete: !result.members.pageInfo.hasNextPage && this.history.length === 0,
        membersEndCursor: result.members.pageInfo.endCursor,
        membersHasNextPage: result.members.pageInfo.hasNextPage,
        membersHasPreviousPage: this.history.length > 0,
        membersPage: this.history.length + 1,
      })
    } catch (error) {
      if (!request.signal.aborted && !owner.signal.aborted && this.request === request) {
        const target = { cursor: this.cursor, history: [...this.history] }
        if (rollback) {
          this.cursor = rollback.cursor
          this.history = rollback.history
        }
        notify.apiError(error, () => {
          if (this.request === request) {
            this.cursor = target.cursor
            this.history = target.history
            void this.load(rollback)
          }
        })
      }
    } finally {
      if (this.request === request) this.store.setState({ membersLoading: false })
    }
  }
}
