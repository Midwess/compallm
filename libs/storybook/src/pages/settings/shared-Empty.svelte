<script lang="ts">
  import { SectionSurface } from '../../components/surfaces'

  import type { Content } from '../../components/shared'

  import ContentValue from '../../components/Content.svelte'

  let {
    title,
    children,
    action,
  }: {
    title: string
    children?: Content
    action?: Content
  } = $props()
</script>

<SectionSurface class="grid gap-3">
  <h3 class="text-xsm leading-5 font-medium"><ContentValue value={title} /></h3>
  {#if children}<p class="text-xs leading-4.5 text-muted-foreground">
      <ContentValue value={children} />
    </p>{:else}{/if}{#if action}<div><ContentValue value={action} /></div>{:else}{/if}
</SectionSurface>
