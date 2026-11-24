<script lang="ts">
  import { SectionSurface } from '../../components/surfaces'

  import { cx } from '../../components/shared'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'

  let {
    children,
    class: className,
  }: {
    children: Content
    class?: string
  } = $props()
</script>

<SectionSurface class={cx('overflow-hidden p-0', className)}><ContentValue value={children} /></SectionSurface>
