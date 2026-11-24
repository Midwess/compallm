<script lang="ts">
  import { resolve } from '$app/paths'
  import { page } from '$app/state'
</script>

<main class="grid min-h-dvh place-content-center gap-4 bg-background p-6 font-sans text-foreground">
  <h1 class="text-2xl font-medium">{page.status === 404 ? 'Page not found' : 'This page could not be loaded'}</h1>
  <a href={resolve('login')} class="text-ring underline">Return to sign in</a>
</main>
