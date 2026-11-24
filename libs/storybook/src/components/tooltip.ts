import Control from './TooltipControl.svelte'
import Root from './TooltipRoot.svelte'
import Trigger from './TooltipTrigger.svelte'
import Content from './TooltipContent.svelte'
export const Tooltip: { Control: typeof Control; Root: typeof Root; Trigger: typeof Trigger; Content: typeof Content } = {
  Control,
  Root,
  Trigger,
  Content,
}
