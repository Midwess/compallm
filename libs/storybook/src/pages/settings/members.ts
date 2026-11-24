import type { SettingsMember } from './types'

export const memberKey = (member: SettingsMember) => member.id
export const memberColumns = [
  { id: 'name' },
  { id: 'email' },
  { id: 'role', class: 'w-1/6', visibilityClassName: 'hidden md:table-cell' },
  { id: 'groups', class: 'w-1/6 lg:w-[calc(100%/6-1.5rem)]', visibilityClassName: 'hidden md:table-cell' },
  { id: 'joined', class: 'w-26', visibilityClassName: 'hidden lg:table-cell' },
  { id: 'actions', class: 'w-12' },
]
export { default as InviteMember } from './members-InviteMember.svelte'
export { default as DirectoryDate } from './members-DirectoryDate.svelte'
export { default as Invitations } from './members-Invitations.svelte'
export { default as MemberRoles } from './members-MemberRoles.svelte'
export { default as MemberRolePicker } from './members-MemberRolePicker.svelte'
export { default as MemberGroups } from './members-MemberGroups.svelte'
export { default as MemberGroupPicker } from './members-MemberGroupPicker.svelte'
export { default as MemberActions } from './members-MemberActions.svelte'
export { default as MemberEmail } from './members-MemberEmail.svelte'
export { default as MemberAvatar } from './members-MemberAvatar.svelte'
export { default as MemberRow } from './members-MemberRow.svelte'
export { default as MemberRows } from './members-MemberRows.svelte'
export { default as InvitationDialog } from './members-InvitationDialog.svelte'
export { default as MemberFilters } from './members-MemberFilters.svelte'
export { default as MembersSettings } from './members-MembersSettings.svelte'
