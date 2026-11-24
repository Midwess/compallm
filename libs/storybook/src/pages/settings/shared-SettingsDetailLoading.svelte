<script lang="ts">
  import { LoadingRegion, Skeleton } from '../../components/loading'

  import { cx } from '../../components/shared'

  import { SettingsContent, Heading, Section, Rows } from './shared'
  import ContentValue from '../../components/Content.svelte'

  let {
    title,
    sections,
    wide = false,
  }: {
    title: string
    sections: string[]
    wide?: boolean
  } = $props()
</script>

<SettingsContent {wide} class={cx('flex flex-col pb-6', wide ? 'pt-16' : 'pt-14')}>
  <Heading class="mb-8 shrink-0"><ContentValue value={title} /></Heading><LoadingRegion
    ready={false}
    pending
    label={`Loading ${title.toLowerCase()}`}
    class="grid gap-6"
  >
    {#each sections as section (section)}<Section title={section}>
        <Rows>
          {#each Array.from({ length: section === 'Permissions' ? 8 : 2 }) as _, index (index)}<div
              class="flex min-h-16 items-center justify-between gap-4 px-4 py-3"
            >
              <Skeleton><span class="h-4 w-24 rounded-md"></span></Skeleton><Skeleton>
                <span class="h-8 w-70 max-w-full rounded-lg"></span>
              </Skeleton>
            </div>{/each}
        </Rows>
      </Section>{/each}
  </LoadingRegion>
</SettingsContent>
