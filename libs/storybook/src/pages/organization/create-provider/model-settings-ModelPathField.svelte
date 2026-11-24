<script lang="ts">
  import { ProviderModelFieldsConstraints } from '@compallm/contract/provider'
  import { FormError } from '../../../components/form'
  import { FormField } from '../../../components/form-field'

  import { FieldHelp, Tooltip } from '../../../components/feedback'

  import { ModelEndpointPreview } from './model-settings'
</script>

<FormField.Root>
  <div class="flex items-center gap-1">
    <FormField.Label class="text-xsm font-normal">Path</FormField.Label><FieldHelp label="About the model endpoint path">
      This path is appended to the provider’s base URL, keeping any base path such as /v1. For example,
      https://api.openai.com/v1 plus /responses becomes https://api.openai.com/v1/responses. Enter only the path, beginning with
      /. Hover or focus the base URL prefix to see the complete endpoint.
    </FieldHelp>
  </div>
  <FormField.PrefixedInput
    name="modelDraft.path"
    required
    autofocus
    spellcheck={false}
    autocapitalize="none"
    autocomplete="off"
    maxlength={ProviderModelFieldsConstraints.path.maxLength}
  >
    {#snippet prefix()}<Tooltip.Root>
        <Tooltip.Trigger
          type="button"
          aria-label="Show provider base URL and complete endpoint"
          class="flex h-full items-center px-2 font-mono text-ui-11 text-muted-foreground
            hover:text-foreground focus-visible:outline-1 focus-visible:outline-ring focus-visible:-outline-offset-2"
        >
          &#123;&#123;base_url&#125;&#125;
        </Tooltip.Trigger><Tooltip.Content class="max-w-[min(32rem,calc(100vw-1rem))] rounded-lg px-3 py-2 text-left">
          <ModelEndpointPreview></ModelEndpointPreview>
        </Tooltip.Content>
      </Tooltip.Root>{/snippet}
  </FormField.PrefixedInput><FormError name="modelDraft.path"></FormError>
</FormField.Root>
