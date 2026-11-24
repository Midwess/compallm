<script lang="ts">
  import { onDestroy, untrack } from 'svelte'
  import { KeyValueEditor } from '../../src/components/key-value-editor'
  import EffortMapping from '../../src/components/EffortMapping.svelte'
  import type { EffortValues } from '../../src/components/effort-mapping'
  import ProviderAvatar from '../../src/components/ProviderAvatar.svelte'
  import RangeSlider from '../../src/components/RangeSlider.svelte'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  let { scenario }: { scenario: string } = $props()
  let headers = $state(
    untrack(() =>
      scenario === 'EmptyHeaders'
        ? []
        : [
            { id: 'client', key: 'X-Client', value: 'Compallm' },
            { id: 'version', key: 'X-Version', value: '1' },
          ],
    ),
  )
  let empty = $state<EffortValues>({})
  let efforts = $state<EffortValues>({ low: 'low', medium: 'medium', high: 'high' })
  let compact = $state<EffortValues>({ low: 'low', medium: 'medium', high: 'high', xhigh: 'xhigh', max: 'max' })
  let range = $state(262144)
  let src = $state<string | null>(null)
  onDestroy(() => {
    if (src) URL.revokeObjectURL(src)
  })
</script>

<Preview>
  {#if scenario === 'Headers' || scenario === 'EmptyHeaders'}<Sample
      label={scenario === 'Headers' ? 'Ordered key/value editing' : 'Key-only entry'}
      wide
    >
      <KeyValueEditor.Root bind:value={headers}>
        <KeyValueEditor.Disclosure open>Headers</KeyValueEditor.Disclosure>
      </KeyValueEditor.Root>
    </Sample>
  {:else if scenario === 'Avatar'}<Sample label="Provider avatar">
      <ProviderAvatar
        provider="openai"
        {src}
        onFileChange={(file) => {
          if (src) URL.revokeObjectURL(src)
          src = file ? URL.createObjectURL(file) : null
        }}
      />
    </Sample>
  {:else if scenario === 'TokenCapacityRuler'}<Sample label="Token ruler with tick feedback" wide>
      <div class="grid gap-2">
        <output class="text-right text-xs text-muted-foreground">{range.toLocaleString()} tokens</output><RangeSlider
          label="Context capacity"
          min={100}
          max={1000000}
          bind:value={range}
          class="text-muted-foreground"
        />
      </div>
    </Sample>
  {:else if scenario === 'CompactEfforts'}<Sample label="Compact efforts with growing glow" wide>
      <EffortMapping variant="compact" bind:value={compact} />
    </Sample>
  {:else}<Sample label="Empty effort names" wide><EffortMapping bind:value={empty} /></Sample><Sample
      label="Saved names with low editing"
      wide
    >
      <EffortMapping defaultFocused="low" bind:value={efforts} />
    </Sample>{/if}
</Preview>
