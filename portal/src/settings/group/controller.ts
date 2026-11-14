import { permissionKey, type PermissionController, type PermissionCheck } from '@compallm/storybook'
import { requirePermissions } from '../../permission/controller'
import {
  createGroupStore,
  type GroupController as Contract,
  type GroupDraft,
  type SettingsState,
} from '@compallm/storybook/pages/settings'
import type { FormStore } from '@compallm/storybook'
import { GroupUpdateInput } from '@compallm/contract/validation'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'

const memberNode = {
  id: true,
  displayName: true,
  email: true,
  available: true,
  effectiveRoleIds: true,
  groupIds: true,
} as const
const pageInfo = { endCursor: true, hasNextPage: true } as const

export class GroupController implements Contract {
  private readonly permissions?: PermissionController
  readonly store = createGroupStore()
  private request?: AbortController
  private memberRequest?: AbortController
  private memberRevision = 0
  private cursor?: string
  private baselineMembers = new Set<string>()
  private baselineRoles = new Set<string>()
  private roleIntents = new Map<string, boolean>()
  private lastRoleSelection = new Set<string>()
  rolesChanged = (ids: string[]) => {
    const next = new Set(ids)
    const previous = this.roleIntents.size ? this.lastRoleSelection : new Set(this.store.form.baseline.draft?.roleIds ?? [])
    for (const id of new Set([...previous, ...next]))
      if (previous.has(id) !== next.has(id)) this.roleIntents.set(id, next.has(id))
    this.lastRoleSelection = next
  }
  private readonly settings: FormStore<SettingsState>
  constructor(settings: FormStore<SettingsState>, permissions?: PermissionController) {
    this.permissions = permissions
    this.settings = settings
  }
  cancel = () => {
    this.request?.abort()
    this.memberRequest?.abort()
    this.memberRequest = undefined
    this.cursor = undefined
    this.roleIntents.clear()
    this.store.setState({ membersLoading: false })
    this.store.form.stop()
    this.request = undefined
  }
  load = async (id: string) => {
    this.cancel()
    const request = new AbortController()
    this.request = request
    this.store.setState({ loading: true, draft: null, query: '', members: [], hasNextPage: false })
    try {
      if (this.permissions) await requirePermissions(this.permissions, [{ action: 'group.read', resourceType: 'Organization' }])
      if (request.signal.aborted || this.request !== request) return
      const reads = (['role.read', 'member.read'] as const).map((action) => ({ action, resourceType: 'Organization' as const }))
      if (this.permissions) await this.permissions.evaluate(reads)
      if (this.request !== request || request.signal.aborted) return
      const allowed = (index: number) =>
        !this.permissions || this.permissions.store.getState().entries[permissionKey(reads[index])]?.status === 'allowed'
      const selection = {
        group: {
          __args: { id },
          node: {
            id: true,
            name: true,
            description: true,
            ...(allowed(0)
              ? { roles: { edges: { node: { id: true, name: true, description: true, color: true, owner: true } } } }
              : {}),
            memberIds: allowed(1),
          },
        },
        ...(allowed(0)
          ? { roles: { edges: { node: { id: true, name: true, description: true, color: true, owner: true } } } }
          : {}),
        ...(allowed(1) ? { members: { __args: { first: 30 }, edges: { node: memberNode }, pageInfo } } : {}),
      } satisfies QueryGenqlSelection
      const result = await graphql(request.signal).query(selection)
      if (this.request !== request || request.signal.aborted) return
      const group = result.group?.node
      const members = result.members ?? { edges: [], pageInfo: { endCursor: null, hasNextPage: false } }
      const roles = result.roles ?? { edges: [] }
      if (!group) {
        this.store.setState({ loading: false, draft: null })
        return
      }
      this.permissions?.setFacts?.([
        { resourceType: 'Group', id, ...(group.roles ? { roleIds: group.roles.edges.map(({ node }) => node.id) } : {}) },
      ])
      this.permissions?.setFacts?.(
        members.edges.flatMap(({ node }) =>
          node.effectiveRoleIds && node.groupIds
            ? [{ resourceType: 'Member' as const, id: node.id, roleIds: node.effectiveRoleIds, groupIds: node.groupIds }]
            : [],
        ),
      )
      this.settings.setState({ readonly: false })
      this.baselineRoles = new Set((group.roles?.edges ?? []).map(({ node }) => node.id))
      this.baselineMembers = new Set(group.memberIds ?? [])
      const draft = {
        id,
        name: group.name,
        description: group.description,
        roleIds: [...this.baselineRoles],
        memberIds: [...this.baselineMembers],
        memberSelection: Object.fromEntries(members.edges.map(({ node }) => [node.id, this.baselineMembers.has(node.id)])),
      }
      this.cursor = members.pageInfo.endCursor ?? undefined
      this.store.setState({
        loading: false,
        draft,
        memberCount: (group.memberIds ?? []).length,
        roles: roles.edges.map(({ node }) => ({ ...node, source: '', builtin: node.owner })),
        members: members.edges.map(({ node: member }) => ({
          id: member.id,
          name: member.displayName || member.email,
          email: member.email,
          available: member.available,
        })),
        hasNextPage: members.pageInfo.hasNextPage,
      })
      this.store.form.reset(this.store.form.values)
    } catch (error) {
      if (this.request === request && !request.signal.aborted) {
        this.store.setState({ loading: false })
        notify.apiError(error, () => {
          if (this.request === request) void this.load(id)
        })
      }
    }
  }
  setQuery = (query: string) => {
    this.store.setState({ query })
    void this.loadMembers(false)
  }
  loadMore = () => {
    if (this.store.getState().hasNextPage) void this.loadMembers(true)
  }
  private async loadMembers(append: boolean) {
    const id = this.store.getState().draft?.id,
      owner = this.request
    if (!id || !owner) return
    this.memberRequest?.abort()
    const request = new AbortController()
    this.memberRequest = request
    const revision = this.memberRevision
    this.store.setState({ membersLoading: true })
    const q = this.store
      .getState()
      .query.trim()
      .replace(/[\\%_]/g, '\\$&')
    const filter = q ? { or: [{ displayName: { ilike: `%${q}%` } }, { email: { ilike: `%${q}%` } }] } : undefined
    try {
      const selection = {
        members: {
          __args: { first: 30, after: append ? this.cursor : undefined, filter },
          edges: { node: memberNode },
          pageInfo,
        },
      } satisfies QueryGenqlSelection
      const result = await graphql(AbortSignal.any([owner.signal, request.signal])).query(selection)
      if (this.request !== owner || this.memberRequest !== request) return
      if (revision !== this.memberRevision) {
        void this.loadMembers(append)
        return
      }
      const options = result.members
      this.permissions?.setFacts?.(
        options.edges.flatMap(({ node }) =>
          node.effectiveRoleIds && node.groupIds
            ? [{ resourceType: 'Member' as const, id: node.id, roleIds: node.effectiveRoleIds, groupIds: node.groupIds }]
            : [],
        ),
      )
      const current = this.store.getState().draft!
      const selectionState = { ...current.memberSelection }
      const confirmed = { ...this.store.form.baseline.draft?.memberSelection }
      for (const { node } of options.edges) {
        const key = node.id
        if (selectionState[key] === confirmed[key]) selectionState[key] = this.baselineMembers.has(key)
        confirmed[key] = this.baselineMembers.has(key)
      }
      const previous = append ? (this.store.getState().members ?? []) : []
      const rows = options.edges.map(({ node: member }) => ({
        id: member.id,
        name: member.displayName || member.email,
        email: member.email,
        available: member.available,
      }))
      this.cursor = options.pageInfo.endCursor ?? undefined
      this.store.setState({
        members: [...previous, ...rows.filter((row) => !previous.some((old) => old.id === row.id))],
        hasNextPage: options.pageInfo.hasNextPage,
      })
      // Newly loaded relationships extend the form baseline without submitting a write.
      for (const { node } of options.edges) this.store.form.set(`draft.memberSelection.${node.id}`, selectionState[node.id])
      this.store.form.set('draft.memberIds', [...this.baselineMembers])
      this.store.form.accept({
        draft: { ...this.store.form.baseline.draft!, memberIds: [...this.baselineMembers], memberSelection: confirmed },
      })
    } catch (error) {
      if (!request.signal.aborted && !owner.signal.aborted)
        notify.apiError(error, () => {
          if (this.request === owner && this.memberRequest === request) void this.loadMembers(append)
        })
    } finally {
      if (this.memberRequest === request) this.store.setState({ membersLoading: false })
    }
  }
  updateGroup = async (draft: GroupDraft, names: string[], signal: AbortSignal): Promise<GroupDraft | false> => {
    const request = this.request,
      id = draft.id
    if (!request || !id) return false
    const input: Record<string, unknown> = {}
    if (names.includes('draft.name')) input.name = draft.name
    if (names.includes('draft.description')) input.description = draft.description
    const roleSet = new Set(draft.roleIds)
    const memberNames = names.filter((name) => name.startsWith('draft.memberSelection.'))
    if (memberNames.length) {
      const ids = memberNames.map((name) => name.slice('draft.memberSelection.'.length))
      input.addMemberIds = ids.filter((id) => draft.memberSelection?.[id])
      input.removeMemberIds = ids.filter((id) => !draft.memberSelection?.[id])
    }
    if (names.includes('draft.roleIds')) {
      const ids = this.roleIntents.size ? [...this.roleIntents.keys()] : [...new Set([...roleSet, ...this.baselineRoles])]
      input.addRoleIds = ids.filter((id) => roleSet.has(id) && !this.baselineRoles.has(id))
      input.removeRoleIds = ids.filter((id) => !roleSet.has(id) && this.baselineRoles.has(id))
    }
    const checks: PermissionCheck[] = []
    if ('name' in input || 'description' in input)
      checks.push({ action: 'group.metadata.update', resourceType: 'Group', resourceId: id })
    for (const [field, action] of [
      ['addRoleIds', 'group.role.assign'],
      ['removeRoleIds', 'group.role.unassign'],
    ] as const)
      for (const roleId of (input[field] as string[] | undefined) ?? [])
        checks.push({ action, resourceType: 'Group', resourceId: id, roleId })
    for (const [field, action] of [
      ['addMemberIds', 'group.member.add'],
      ['removeMemberIds', 'group.member.remove'],
    ] as const)
      for (const memberId of (input[field] as string[] | undefined) ?? [])
        checks.push({ action, resourceType: 'Group', resourceId: id, memberId })
    if (this.permissions) await requirePermissions(this.permissions, checks)
    if (signal.aborted || request.signal.aborted || this.request !== request) return false
    const parsed = GroupUpdateInput.parse(input)
    const selection = {
      groupUpdate: {
        __args: { id, input: parsed },
        node: {
          id: true,
          name: true,
          description: true,
          memberIds:
            !this.permissions ||
            this.permissions.store.getState().entries[permissionKey({ action: 'member.read', resourceType: 'Organization' })]
              ?.status === 'allowed',
          ...(!this.permissions ||
          this.permissions.store.getState().entries[permissionKey({ action: 'role.read', resourceType: 'Organization' })]
            ?.status === 'allowed'
            ? { roles: { edges: { node: { id: true, name: true, description: true, color: true } } } }
            : {}),
        },
      },
    } satisfies MutationGenqlSelection
    if (memberNames.length) this.memberRevision++
    const {
      groupUpdate: { node },
    } = await graphql(AbortSignal.any([signal, request.signal]))
      .mutation(selection)
      .finally(() => {
        if (memberNames.length) this.memberRevision++
      })
    if (this.request !== request || request.signal.aborted) return false
    this.permissions?.setFacts?.([
      { resourceType: 'Group', id, ...(node.roles ? { roleIds: node.roles.edges.map(({ node }) => node.id) } : {}) },
    ])
    this.baselineMembers = new Set(node.memberIds ?? [...this.baselineMembers])
    if (names.includes('draft.roleIds'))
      for (const [id, desired] of this.roleIntents) {
        if (desired === roleSet.has(id)) this.roleIntents.delete(id)
      }
    if (node.roles) this.baselineRoles = new Set(node.roles.edges.map(({ node }) => node.id))
    this.store.setState({ memberCount: (node.memberIds ?? [...this.baselineMembers]).length })
    this.settings.setState((state) => ({
      groups: state.groups.some((group) => group.id === id)
        ? state.groups.map((group) => (group.id === id ? { ...group, name: node.name, description: node.description } : group))
        : [...state.groups, { id, name: node.name, description: node.description, roleIds: [], memberIds: [], color: '' }],
    }))
    return {
      ...draft,
      name: node.name,
      description: node.description,
      roleIds: [...this.baselineRoles],
      memberIds: [...this.baselineMembers],
      memberSelection: Object.fromEntries(
        Object.keys(draft.memberSelection ?? {}).map((id) => [id, this.baselineMembers.has(id)]),
      ),
    }
  }
}
