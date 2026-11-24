<script lang="ts">
  import { toStyle } from '../page-state.svelte'

  let {
    color,
  }: {
    color: string
  } = $props()
</script>

<span aria-hidden="true" class="inline-block size-1.5 shrink-0 rounded-full" style={toStyle({ backgroundColor: color })}></span>
