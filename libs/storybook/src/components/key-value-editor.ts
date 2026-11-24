import Root from './KeyValueRoot.svelte'
import Content from './KeyValueContent.svelte'
import Disclosure from './KeyValueDisclosure.svelte'
export const KeyValueEditor: { Root: typeof Root; Content: typeof Content; Disclosure: typeof Disclosure } = {
  Root,
  Content,
  Disclosure,
}
export type { KeyValueEntry } from './key-value-context'
