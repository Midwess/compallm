<script lang="ts">
  import { Select } from '../../components/dropdowns'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  let {
    name,
    options,
  }: {
    name: string
    options: string[]
  } = $props()
  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let value = $derived(readStore(__settings.store, (s) => s.filters[name] ?? 'all'))
</script>

<Select.Control
  aria-label={name}
  class="min-w-36 flex-1"
  {value}
  options={[{ value: 'all', label: `All ${name.toLowerCase()}` }, ...options.map((label) => ({ value: label, label }))]}
  onValueChange={(v) => c.pages.search.setFilter(name, v)}
></Select.Control>
