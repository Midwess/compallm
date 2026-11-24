<script lang="ts">
  import { setCollectionRow, getCollectionContext } from '../context'
  import type { Snippet } from 'svelte'
  let { rowKey, index, children }: { rowKey: string; index: number; children?: Snippet } = $props()
  const owner = getCollectionContext()
  setCollectionRow({
    get key() {
      return rowKey
    },
    get index() {
      return index
    },
    forms: owner.forms,
  })
</script>

{@render children?.()}
