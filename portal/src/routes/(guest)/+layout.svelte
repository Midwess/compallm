<script lang="ts">
  import type { AuthMode } from '@compallm/storybook/pages/auth'
  import { page } from '$app/state'
  import RouteGate from '../../RouteGate.svelte'
  let { children } = $props()
  let mode: AuthMode = $derived(page.url.pathname.startsWith('/signup') || page.url.pathname === '/' ? 'signup' : 'signin')
</script>

<RouteGate {mode}>{@render children()}</RouteGate>
