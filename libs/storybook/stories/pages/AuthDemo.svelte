<script lang="ts">
  import { untrack } from 'svelte'
  import type { AuthMode } from '../../src/pages/auth/types'
  import type { AuthDemoOptions } from './demo-auth-controller'
  import AuthDemoScope from './AuthDemoScope.svelte'
  let { mode = 'signin', ...options }: AuthDemoOptions & { popup?: boolean } = $props()
  let navigated = $state(false)
  let active = $state<AuthMode>(untrack(() => mode))
</script>

{#key active}<AuthDemoScope
    {...options}
    initial={navigated ? undefined : options.initial}
    draft={navigated ? undefined : options.draft}
    mode={active}
    onNavigate={(mode) => {
      navigated = true
      active = mode
    }}
  />{/key}
