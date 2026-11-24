<script lang="ts">
  import { ModelSection, PriceRow } from './model-sections'
  import { modelTones } from './model-sections'
</script>

<ModelSection title="Custom Pricing">
  {#snippet action()}<span class="text-xs text-muted-foreground">USD / 1M tokens</span>{/snippet}{#snippet help()}<div
      class="grid gap-2"
    >
      <p>
        These rates describe the model’s input, cached-input, and output costs in US dollars per one million tokens. They do not
        change what the provider charges.
      </p>
      <p>
        Enable a row to set a custom rate from $0 to $10,000. An explicit $0 means free. Uncheck it to use the retained template
        rate; an unknown rate stays unset and pauses autosave.
      </p>
      <p>
        For example, 1,000 input tokens at $2 per million cost $0.002 before output costs. Comparison hints are calculated; they
        do not link or change the prices.
      </p>
    </div>{/snippet}
  <div class="grid min-w-0 gap-3 rounded-control bg-inset p-4">
    <div class="grid gap-2">
      <h3 class={`text-xs ${modelTones.input}`}>Input</h3>
      <PriceRow label="Standard" name="inputPrice"></PriceRow><PriceRow label="Cache read" name="cachedPrice"></PriceRow>
    </div>
    <div class="border-t border-divider"></div>
    <div class="grid gap-2">
      <h3 class={`text-xs ${modelTones.output}`}>Output</h3>
      <PriceRow label="Generated" name="outputPrice"></PriceRow>
    </div>
  </div>
</ModelSection>
