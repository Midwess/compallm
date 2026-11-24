<script lang="ts">
  import { FieldHelp } from '../../../components/feedback'

  import type { Content } from '../../../components/shared'

  import ContentValue from '../../../components/Content.svelte'

  let {
    title,
    children,
    action,
    description,
    help,
  }: {
    title: string
    children: Content
    action?: Content
    description?: string
    help?: Content
  } = $props()

  const id = $props.id()
</script>

<section aria-labelledby={id} class="grid min-w-0 gap-2">
  <div class="flex min-h-6 items-center justify-between gap-3 px-4">
    <div class="flex min-w-0 items-center gap-1">
      <h2 {id} class="text-xsm font-medium"><ContentValue value={title} /></h2>
      {#if help}<FieldHelp label={`About ${title.toLowerCase()}`}><ContentValue value={help} /></FieldHelp>{:else}{/if}
    </div>
    <ContentValue value={action} />
  </div>
  {#if description}<p class="px-4 text-xs text-muted-foreground">
      <ContentValue value={description} />
    </p>{:else}{/if}<ContentValue value={children} />
</section>
