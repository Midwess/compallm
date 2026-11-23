<script lang="ts">
  import { ScaffoldCheckStore, type ScaffoldCheck$result } from '@compallm/graphql-svelte'
  const fixture = { __typename: 'Query' } satisfies ScaffoldCheck$result
</script>

<svelte:head><title>Compallm</title></svelte:head>
<main
  class="min-h-dvh bg-background font-sans text-foreground"
  data-query={ScaffoldCheckStore.name}
  aria-label={`Compallm ${fixture.__typename}`}
></main>
