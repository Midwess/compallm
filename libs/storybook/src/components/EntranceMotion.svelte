<script lang="ts">
  import { entranceMotion } from './motion'
  import type { ElementProps } from './shared'
  let { children, order = 0, subtle = false, ...props }: ElementProps & { order?: number; subtle?: boolean } = $props()
</script>

<div {...props} use:entranceMotion={{ order, subtle }}>{@render children?.()}</div>
