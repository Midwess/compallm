import { type PreferenceDraft } from './values'
import type {
  PreferenceDestination$options as PreferenceDestination,
  PreferenceSidebarCategory$options as PreferenceSidebarCategory,
} from '@compallm/contract/houdini'
export type Layout = PreferenceDraft['sidebarLayout']
export type Drag = {
  category: PreferenceSidebarCategory
  item?: PreferenceDestination
}
export { default as CustomizeSidebar } from './sidebar-layout-CustomizeSidebar.svelte'
