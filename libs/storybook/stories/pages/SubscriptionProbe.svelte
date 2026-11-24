<script lang="ts">
  import { untrack } from 'svelte'
  let { label, value }: { label: string; value: unknown } = $props()
  let updates = $state(0)
  $effect(() => {
    void value
    untrack(() => updates++)
  })
</script>

<p>{label}: <output aria-label={`${label} updates`}>{updates}</output></p>
