import {
  SettingsGroupGroupAndRolesAndMembersStore,
  SettingsGroupMembersStore,
  SettingsGroupGroupUpdateStore,
} from '@compallm/contract/houdini'
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

import { graphqlClient, requireData } from '../../graphql-client'
import { notify } from '../../notifications/toast'

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
    const retained = this.store.getState().draft?.id === id ? this.store.getState().draft : null
    if (retained && (this.store.form.dirty || this.store.form.pending)) return
    this.cancel()
    const request = new AbortController()
    this.request = request
    if (this.seeded === id) {
      this.seeded = undefined
      if (this.prepared) this.adopt(this.prepared, id)
      this.prepared = undefined
      return
    }
    this.store.setState({
      loading: true,
      ...(retained
        ? {}
        : { membersLoaded: false, draft: null, query: '', members: [], hasNextPage: false, membersLoadError: false }),
    })
    try {
      if (this.permissions) await requirePermissions(this.permissions, [{ action: 'group.read', resourceType: 'Organization' }])
      if (request.signal.aborted || this.request !== request) return
      const reads = (['role.read', 'member.read'] as const).map((action) => ({ action, resourceType: 'Organization' as const }))
      if (this.permissions) await this.permissions.evaluate(reads)
      if (this.request !== request || request.signal.aborted) return
      const allowed = (index: number) =>
        !this.permissions || this.permissions.store.getState().entries[permissionKey(reads[index])]?.status === 'allowed'

      const result = await new SettingsGroupGroupAndRolesAndMembersStore()
        .fetch({
          variables: { groupId: id, includeGroupNode: allowed(0), includeGroupNodeMemberIds: allowed(1) },
          fetch: graphqlClient.fetch(request.signal),
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
      if (this.request !== request || request.signal.aborted) return
      if (retained && (this.store.form.dirty || this.store.form.pending)) return
      this.adopt(result, id)
    } catch (error) {
      if (this.request === request && !request.signal.aborted) {
        this.store.setState({ loading: false })
        notify.apiError(error, () => {
          if (this.request === request) void this.load(id)
        })
      }
    }
  }
  private adopt(result: import('@compallm/contract/houdini').SettingsGroupGroupAndRolesAndMembers$result, id: string) {
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
      membersLoaded: true,
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
  }
  seed(result: import('@compallm/contract/houdini').SettingsGroupGroupAndRolesAndMembers$result, id: string) {
    this.adopt(result, id)
    this.prepared = result
    this.seeded = id
  }
  private seeded?: string
  private prepared?: import('@compallm/contract/houdini').SettingsGroupGroupAndRolesAndMembers$result
  setQuery = (query: string) => {
    this.store.setState({ query })
    void this.loadMembers(false)
  }
  loadMore = () => {
    if (this.store.getState().hasNextPage && !this.store.getState().membersLoading) void this.loadMembers(true)
  }
  private async loadMembers(append: boolean) {
    const id = this.store.getState().draft?.id,
      owner = this.request
    if (!id || !owner) return
    this.memberRequest?.abort()
    const request = new AbortController()
    this.memberRequest = request
    const revision = this.memberRevision
    this.store.setState({ membersLoading: true, membersLoadError: false })
    const q = this.store
      .getState()
      .query.trim()
      .replace(/[\\%_]/g, '\\$&')
    const filter = q ? { or: [{ displayName: { ilike: `%${q}%` } }, { email: { ilike: `%${q}%` } }] } : undefined
    try {
      const result = await new SettingsGroupMembersStore()
        .fetch({
          variables: { membersAfter: append ? this.cursor : undefined, membersFilter: filter },
          fetch: graphqlClient.fetch(AbortSignal.any([owner.signal, request.signal])),
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
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
      if (!request.signal.aborted && this.memberRequest === request) this.store.setState({ membersLoadError: true })
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

    if (memberNames.length) this.memberRevision++
    const {
      groupUpdate: { node },
    } = await new SettingsGroupGroupUpdateStore()
      .mutate(
        {
          groupUpdateId: id,
          groupUpdateInput: parsed,
          includeGroupUpdateNodeMemberIds:
            !this.permissions ||
            this.permissions.store.getState().entries[permissionKey({ action: 'member.read', resourceType: 'Organization' })]
              ?.status === 'allowed',
          includeGroupUpdateNode:
            !this.permissions ||
            this.permissions.store.getState().entries[permissionKey({ action: 'role.read', resourceType: 'Organization' })]
              ?.status === 'allowed',
        },
        { fetch: graphqlClient.fetch(AbortSignal.any([signal, request.signal])) },
      )
      .then(requireData)
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
