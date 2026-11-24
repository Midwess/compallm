import GearIcon from 'phosphor-svelte/lib/GearIcon'
import ChatTextIcon from 'phosphor-svelte/lib/ChatTextIcon'
import CodeIcon from 'phosphor-svelte/lib/CodeIcon'
import UserIcon from 'phosphor-svelte/lib/UserIcon'
import KeyIcon from 'phosphor-svelte/lib/KeyIcon'
import InfoIcon from 'phosphor-svelte/lib/InfoIcon'
import UsersIcon from 'phosphor-svelte/lib/UsersIcon'

import ChartLineIcon from 'phosphor-svelte/lib/ChartLineIcon'
import ShieldCheckIcon from 'phosphor-svelte/lib/ShieldCheckIcon'

export const personal = [
  { page: 'general', label: 'General', Icon: GearIcon },
  { page: 'messaging', label: 'Messaging', Icon: ChatTextIcon },
  { page: 'code', label: 'Coding', Icon: CodeIcon },
] as const
export const account = [
  { page: 'profile', label: 'Profile', Icon: UserIcon },
  { page: 'access', label: 'Access', Icon: KeyIcon },
  { page: 'security', label: 'Security & access', Icon: ShieldCheckIcon },
] as const
export const organization = [
  { page: 'organization', label: 'Info', Icon: InfoIcon },
  { page: 'organization-onboarding', label: 'Onboarding', Icon: GearIcon },
  { page: 'members', label: 'Members', Icon: UsersIcon },
  { page: 'usage-limits', label: 'Usage & limits', Icon: ChartLineIcon },
] as const
export const category = '[&>details>summary]:text-xs [&>details>summary]:font-normal'
export const folder = '[&>details>summary>span:first-child]:size-3'
export const item = 'font-normal [&>span:first-child]:size-3 [&>span:first-child>svg]:size-3'
export { default as SettingsNavigation } from './sidebar-SettingsNavigation.svelte'
export { default as NavigationPermission } from './sidebar-NavigationPermission.svelte'
