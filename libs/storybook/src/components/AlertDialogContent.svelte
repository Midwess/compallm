<script lang="ts">
  import Content from './DialogContent.svelte'
  import { getDialog } from './dialog-context'
  import { cx } from './shared'
  import type { ComponentProps } from 'svelte'
  let { class: className, ...props }: ComponentProps<typeof Content> = $props()
  const scope = getDialog()
</script>

<Content {...props} role="alertdialog" aria-describedby={scope.descriptionId} class={cx('w-sm', className)} />
