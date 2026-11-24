<script lang="ts">
  import { effectiveModelPrice, priceEnabledFields, type ModelPriceField } from './model-options'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  let {
    name,
  }: {
    name: ModelPriceField
  } = $props()
  const __provider = getCreateProviderController()
  let values = $derived(
    readStore(
      __provider.store,
      (s) =>
        s.modelDraft
          ? { input: effectiveModelPrice(s.modelDraft, 'inputPrice'), rate: effectiveModelPrice(s.modelDraft, name) }
          : {},
      [
        'modelDraft.inputPrice',
        `modelDraft.${name}`,
        'modelDraft.inputPriceEnabled',
        `modelDraft.${priceEnabledFields[name]}`,
        'modelDraft.catalogPrices',
      ],
    ),
  )
  let base = $derived(Number(values.input))
  let rate = $derived(Number(values.rate))
  let hint = $derived(
    values.input && values.rate && base > 0 && Number.isFinite(rate)
      ? name === 'cachedPrice'
        ? `${Math.round(Math.abs(1 - rate / base) * 100)}% ${rate <= base ? 'less' : 'more'}`
        : name === 'outputPrice'
          ? `${Number((rate / base).toFixed(2))}× input`
          : ''
      : '',
  )
</script>

<span class="hidden shrink-0 text-ui-11 text-muted-foreground sm:block">{hint}</span>
