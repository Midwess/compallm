<script lang="ts">
  import { Skeleton } from '../../components/loading'

  import { SettingsDivider } from '../../components/settings'

  import { SettingsRow } from '../../components/content'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'

  let {
    label,
    description,
    children,
    last = false,
  }: {
    label: Content
    description?: Content
    children?: Content
    last?: boolean
  } = $props()
</script>

<SettingsRow {label} {description} class="gap-4 bg-transparent p-4 @max-[22rem]:flex-col @max-[22rem]:items-stretch">
  {#if children}<Skeleton>
      <div
        class="min-w-0 max-w-[65%] shrink @max-[22rem]:flex @max-[22rem]:w-full @max-[22rem]:max-w-full
@max-[22rem]:justify-end @max-[22rem]:[&>input]:w-full @max-[22rem]:*:data-[ui=dropdown]:w-full"
      >
        <ContentValue value={children} />
      </div>
    </Skeleton>{:else}{/if}
</SettingsRow>{#if !last}<SettingsDivider class="mx-4 mt-[-0.5px]"></SettingsDivider>{:else}{/if}
