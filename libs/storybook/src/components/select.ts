import Control from './Dropdown.svelte'
import Root from './SelectRoot.svelte'
import Trigger from './SelectTrigger.svelte'
import Value from './SelectValue.svelte'
import Content from './SelectContent.svelte'
import Item from './SelectItem.svelte'
import Group from './SelectGroup.svelte'
import GroupHeading from './SelectGroupHeading.svelte'
export const Select: {
  Control: typeof Control
  Root: typeof Root
  Trigger: typeof Trigger
  Value: typeof Value
  Content: typeof Content
  Item: typeof Item
  Group: typeof Group
  GroupHeading: typeof GroupHeading
} = { Control, Root, Trigger, Value, Content, Item, Group, GroupHeading }
