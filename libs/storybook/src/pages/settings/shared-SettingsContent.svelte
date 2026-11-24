<script lang="ts">
  import { cx } from '../../components/shared'

  import type { ElementProps } from '../../components/shared'

  let {
    children,
    wide = false,
    class: className,
    ...props
  }: ElementProps & {
    wide?: boolean
  } = $props()
</script>

<article
  {...props}
  class={cx(
    'mx-auto min-w-0 py-16',
    wide ? 'w-full px-6 lg:px-14 [&>div:first-child]:px-0' : 'w-[calc(100%-2.75rem)] max-w-160',
    className,
  )}
>
  {@render children?.()}
</article>
