<script lang="ts">
  import { ModelSection, ContextExample, LimitRow } from './model-sections'
</script>

<ModelSection title="Context windows">
  {#snippet help()}<div class="grid gap-2">
      <p>
        A token is a piece of text. The context window is the total space for a request and its reply, including instructions
        and conversation history.
      </p>
      <p>
        Actual input + actual output must fit inside that window. With 128k context, a 16k reply leaves at most 112k for input.
      </p>
      <p>Input and output limits are separate ceilings. Their configured values do not have to add up to the context window.</p>
      <p>
        Uncheck an override to use the provider default. Increasing a value cannot expand the upstream model’s real capacity.
      </p>
    </div>{/snippet}
  <div class="grid min-w-0 gap-3 rounded-control bg-inset px-3 py-4">
    <ContextExample></ContextExample>
    <div class="mx-1 border-t border-divider"></div>
    <LimitRow kind="context" label="Context window" valueKey="maxContextTokens" enabledKey="contextLimitEnabled"></LimitRow>
    <div class="mx-1 border-t border-divider"></div>
    <LimitRow kind="input" label="Input limit" valueKey="maxInputTokens" enabledKey="inputLimitEnabled"></LimitRow>
    <div class="mx-1 border-t border-divider"></div>
    <LimitRow kind="output" label="Output limit" valueKey="maxOutputTokens" enabledKey="outputLimitEnabled"></LimitRow>
  </div>
</ModelSection>
