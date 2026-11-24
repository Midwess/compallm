<script lang="ts">
  import type { Snippet } from 'svelte'
  import { setLoadingContext, getLoadingContext } from '../context'
  import { optionalContext } from './form-context.svelte'
  import { loadingPresentation } from './loading-presentation.svelte'
  let { ready, refreshing = false, children }: { ready: boolean; refreshing?: boolean; children?: Snippet } = $props()
  const parent = optionalContext(getLoadingContext)
  const presentation = loadingPresentation(
    () => !ready || refreshing,
    () => !ready,
  )
  const inherited = () => ready && !refreshing && !!parent?.refreshing
  setLoadingContext({
    get initial() {
      return !ready || presentation.initial
    },
    get visible() {
      return inherited() ? !!parent?.visible : presentation.visible
    },
    get refreshing() {
      return refreshing || inherited() || (!presentation.initial && presentation.visible)
    },
  })
</script>

{@render children?.()}
