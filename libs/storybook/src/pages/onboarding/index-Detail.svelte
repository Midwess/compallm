<script lang="ts">
  import { SectionSurface } from '../../components/surfaces'

  import ContentValue from '../../components/Content.svelte'

  let {
    label,
    value,
  }: {
    label: string
    value: string
  } = $props()
</script>

<SectionSurface class="flex min-h-15 flex-col justify-center px-4 py-2.5">
  <p class="text-sm leading-5 text-muted-foreground"><ContentValue value={label} /></p>
  <p class="mt-0.5 wrap-break-word text-base leading-6 font-medium"><ContentValue {value} /></p>
</SectionSurface>
