<script lang="ts">
  import { loadingMotion } from './motion'
  import type { ElementProps } from './shared'
  let { children, ...props }: ElementProps = $props()
</script>

<span {...props} use:loadingMotion>{@render children?.()}</span>
