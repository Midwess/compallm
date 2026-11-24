<script lang="ts">
  import { SettingsSectionHeader } from '../../components/settings'
  import { SettingsSectionTitle, SettingsSectionDescription } from '../../components/settings'

  import { cx } from '../../components/shared'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'

  let {
    title,
    description,
    children,
    action,
    class: className,
  }: {
    title: Content
    description?: Content
    children: Content
    action?: Content
    class?: string
  } = $props()
</script>

<section class={cx('grid min-w-0 gap-4', className)}>
  <div class="flex items-start justify-between gap-3 px-4">
    <SettingsSectionHeader class="min-w-0 flex-1">
      <SettingsSectionTitle><ContentValue value={title} /></SettingsSectionTitle>{#if description}<SettingsSectionDescription>
          <ContentValue value={description} />
        </SettingsSectionDescription>{:else}{/if}
    </SettingsSectionHeader><ContentValue value={action} />
  </div>
  <ContentValue value={children} />
</section>
