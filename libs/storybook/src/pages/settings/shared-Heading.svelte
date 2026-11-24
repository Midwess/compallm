<script lang="ts">
  import { SettingsPageTitle } from '../../components/settings'

  import { cx } from '../../components/shared'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'

  let {
    children,
    description,
    action,
    class: className,
  }: {
    children: Content
    description?: Content
    action?: Content
    class?: string
  } = $props()
</script>

<div class={cx('mb-10 px-4', className)}>
  <div class="flex flex-wrap items-center justify-between gap-4">
    <SettingsPageTitle><ContentValue value={children} /></SettingsPageTitle><ContentValue value={action} />
  </div>
  {#if description}<p class="mt-2 text-xsm leading-5.5 text-muted-foreground">
      <ContentValue value={description} />
    </p>{:else}{/if}
</div>
