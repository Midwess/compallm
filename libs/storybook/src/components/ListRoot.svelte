<script lang="ts">
  import { onDestroy } from 'svelte'
  import { setCollectionContext } from '../context'
  import { CollectionForms } from './collection-forms'
  import { scrollOwner } from './collection'
  import { cx, type ElementProps } from './shared'
  let {
    children,
    class: className,
    forms,
    scrollElement,
    ...props
  }: ElementProps & { forms?: CollectionForms; scrollElement?: HTMLElement | null } = $props()
  const owned = new CollectionForms()
  let owner = $state<HTMLElement | null>(null)
  setCollectionContext({
    get forms() {
      return forms ?? owned
    },
    get scrollElement() {
      return scrollElement ?? owner
    },
  })
  onDestroy(owned.dispose)
  function attach(node: HTMLElement) {
    owner = scrollOwner(node)
  }
</script>

<div {...props} data-slot="list-container" class={cx('min-w-0', className)} use:attach>{@render children?.()}</div>
