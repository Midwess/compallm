import { animation } from '../../../components/motion'

export const ownerExplanation = 'The Owner role cannot be edited or deleted.'
export const policyPanelVariants = {
  visible: { opacity: 1, x: 0, display: 'block', transitionEnd: { display: 'block' } },
  visualHidden: { opacity: 0, x: -animation.panelDistance, transitionEnd: { display: 'none' } },
  cedarHidden: { opacity: 0, x: animation.panelDistance, transitionEnd: { display: 'none' } },
}
export { default as RoleHeading } from './Heading.svelte'
export { default as EditingMode } from './index-EditingMode.svelte'
export { default as CedarEditor } from './index-CedarEditor.svelte'
export { default as RoleDetails } from './index-RoleDetails.svelte'
export { default as RoleEditorContent } from './index-RoleEditorContent.svelte'
export { default as RoleEditorSettings } from './index-RoleEditorSettings.svelte'
