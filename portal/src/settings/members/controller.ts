import { GroupUpdateInput } from '@compallm/contract/validation'
import type { PermissionCheck } from '@compallm/storybook'
import type { ProfileAuth } from '../profile/controller'
import { permissionKey, type PermissionController, type FormStore } from '@compallm/storybook'
import type { SettingsState } from '@compallm/storybook/pages/settings'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
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
  private readonly auth: ProfileAuth
  constructor(store: FormStore<SettingsState>, permissions?: PermissionController, auth: ProfileAuth = {}) {
    this.auth = auth
    this.store = store
    this.permissions = permissions
  }
  private readonly groupWrites = new Set<string>()
  setMemberGroups = async (memberId: string, groupIds: string[]): Promise<string[] | false> => {
    const owner = this.lifecycle
    const member = this.store.getState().members.find((member) => member.id === memberId)
    if (!owner || !member || this.groupWrites.has(memberId)) return false
    this.request?.abort()
    this.store.setState({ membersLoading: false })
    this.groupWrites.add(memberId)
    let confirmed = (member.groups ?? []).map((group) => group.id)
    try {
      const desired = new Set(groupIds)
      for (const id of [...new Set([...confirmed, ...desired])].sort()) {
        const add = desired.has(id)
        if (confirmed.includes(id) === add) continue
        if (this.permissions)
          await requirePermissions(this.permissions, [
            { action: add ? 'group.member.add' : 'group.member.remove', resourceType: 'Group', resourceId: id, memberId },
          ])
        if (owner.signal.aborted) return false
        const input = GroupUpdateInput.parse(add ? { addMemberIds: [memberId] } : { removeMemberIds: [memberId] })
        const selection = {
          groupUpdate: { __args: { id, input }, node: { id: true, memberIds: true } },
        } satisfies MutationGenqlSelection
        const result = await graphql(owner.signal).mutation(selection)
        if (owner.signal.aborted || this.lifecycle !== owner) return false
        confirmed = result.groupUpdate.node.memberIds?.includes(memberId)
          ? [...new Set([...confirmed, id])]
          : confirmed.filter((group) => group !== id)
        const catalog = [...this.store.getState().groups, ...(member.groups ?? [])]
        this.store.setState((state) => ({
          members: state.members.map((row) =>
            row.id === memberId
              ? {
                  ...row,
                  groups: confirmed.flatMap((id) => {
                    const group = catalog.find((group) => group.id === id)
                    return group ? [{ id: group.id, name: group.name, description: group.description }] : []
                  }),
                }
              : row,
          ),
        }))
        const directRole = this.store.getState().members.find((row) => row.id === memberId)?.role
        if (directRole)
          this.permissions?.setFacts?.([
            {
              resourceType: 'Member',
              id: memberId,
              groupIds: confirmed,
              roleIds: [
                ...new Set([
                  directRole.id,
                  ...this.store
                    .getState()
                    .groups.filter((group) => confirmed.includes(group.id))
                    .flatMap((group) => group.roleIds),
                ]),
              ],
            },
          ])
      }
      return confirmed
    } catch (error) {
      if (!owner.signal.aborted) notify.error(error)
      return false
    } finally {
      this.groupWrites.delete(memberId)
    }
  }
  private readonly roleWrites = new Set<string>()
  setMemberRole = async (memberId: string, id: string): Promise<void | false> => {
    const owner = this.lifecycle
    const member = this.store.getState().members.find((row) => row.id === memberId)
    if (!owner || !member || !member.role || this.roleWrites.has(memberId)) return false
    const actorId = this.auth.store?.getState().currentUser?.userId ?? this.store.getState().currentMemberId
    if (memberId === actorId) return member.role.id === id ? undefined : false
    const checks: PermissionCheck[] = [
      { action: 'member.read', resourceType: 'Organization' },
      { action: 'role.read', resourceType: 'Organization' },
    ]
    if (id)
      checks.push(
        { action: 'member.role.assign', resourceType: 'Member', resourceId: memberId, roleId: id },
        { action: 'role.assign', resourceType: 'Role', resourceId: id },
      )
    if (member.role.id !== id)
      checks.push(
        { action: 'member.role.unassign', resourceType: 'Member', resourceId: memberId, roleId: member.role.id },
        { action: 'role.unassign', resourceType: 'Role', resourceId: member.role.id },
      )
    this.roleWrites.add(memberId)
    this.request?.abort()
    this.store.setState({ membersLoading: false })
    try {
      if (this.permissions) await requirePermissions(this.permissions, checks)
      if (owner.signal.aborted) return false
      const role = (
        await graphql(owner.signal).mutation({
          roleAssign: { __args: { id, memberId }, node: { id: true, name: true, description: true, color: true, owner: true } },
        } satisfies MutationGenqlSelection)
      ).roleAssign.node
      if (owner.signal.aborted || this.lifecycle !== owner) return false
      this.store.setState((state) => ({
        members: state.members.map((row) => (row.id === memberId ? { ...row, role } : row)),
      }))
      void this.load()
    } catch (error) {
      if (!owner.signal.aborted) notify.error(error)
      return false
    } finally {
      this.roleWrites.delete(memberId)
    }
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
        me: { node: { id: true, organizationOwner: true } },
        organization: { node: { ownerMemberIds: true } },
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
        actorIsOwner: result.me?.node.organizationOwner ?? false,
        ownerMemberIds: result.organization?.node.ownerMemberIds ?? [],
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
