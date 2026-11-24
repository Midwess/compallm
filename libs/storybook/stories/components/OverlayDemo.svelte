<script lang="ts">
  import { untrack } from 'svelte'
  import { AppLoadingOverlay } from '../../src'
  let { scenario = 'Loading' }: { scenario?: string } = $props()
  let error = $state<string | undefined>(
    untrack(() => (scenario === 'Failure' ? 'Could not load your preferences.' : undefined)),
  )
</script>

<AppLoadingOverlay active {error} reducedMotion={scenario === 'ReducedMotion'} onRetry={() => (error = undefined)} />
