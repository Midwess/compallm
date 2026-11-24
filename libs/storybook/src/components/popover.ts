import Control from './PopoverControl.svelte'
import Root from './PopoverRoot.svelte'
import Trigger from './PopoverTrigger.svelte'
import Content from './PopoverContent.svelte'
export const Popover: { Control: typeof Control; Root: typeof Root; Trigger: typeof Trigger; Content: typeof Content } = {
  Control,
  Root,
  Trigger,
  Content,
}
