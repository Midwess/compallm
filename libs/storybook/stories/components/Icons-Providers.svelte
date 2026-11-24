<script lang="ts">
  import { ProviderIcon } from '../../src/components/provider-icons'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  {#each ['openai', 'anthropic', 'gemini', 'deepseek', 'groq', 'mistral'] as const as provider (provider)}<Sample
      label={provider}
    >
      <Row>
        <ProviderIcon {provider} class="size-4"></ProviderIcon><ProviderIcon {provider} class="size-6"
        ></ProviderIcon><ProviderIcon {provider} class="size-8"></ProviderIcon>
      </Row>
    </Sample>{/each}
</Preview>
