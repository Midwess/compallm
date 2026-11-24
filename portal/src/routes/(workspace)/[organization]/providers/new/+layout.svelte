<script lang="ts">
  let { children } = $props()
  import ProviderScope from '../../../../../ProviderScope.svelte'
</script>

<ProviderScope />

{@render children?.()}
