<script lang="ts">
  import { SectionSurface } from '../../../components/surfaces'

  import CheckIcon from 'phosphor-svelte/lib/CheckIcon'
  import XCircleIcon from 'phosphor-svelte/lib/XCircleIcon'
  import LinkSimpleIcon from 'phosphor-svelte/lib/LinkSimpleIcon'
  import { Button } from '../../../components/buttons'

  import { Tooltip } from '../../../components/feedback'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let result = $derived(readStore(__provider.store, (state) => state.test))
  let endpoint = $derived(readStore(__provider.store, (state) => state.endpointUrl))
  let busy = $derived(readStore(__provider.store, (state) => state.submitting))
</script>

<SectionSurface class="grid gap-2">
  <p class="flex min-w-0 items-center gap-2 text-xs text-ring">
    <LinkSimpleIcon aria-hidden="true" class="size-3.5 shrink-0"></LinkSimpleIcon><span class="truncate">
      {endpoint || 'Configure the connection endpoint'}
    </span>
  </p>
  <div class="flex flex-wrap items-center gap-3">
    <Button
      class="h-7 px-3 text-xs"
      disabled={busy}
      loading={result.status === 'testing'}
      onclick={() => void controller.onTestConnection()}
    >
      Test connection
    </Button><span role="status" aria-live="polite">
      {#if result.status === 'passed'}<span class="flex items-center gap-1.5 text-xs text-success">
          <CheckIcon aria-hidden="true" class="size-3.5"></CheckIcon>Check passed
        </span>{:else}{#if result.status === 'failed'}<Tooltip.Control content={result.message}>
            <button
              type="button"
              class="flex cursor-help items-center gap-1.5 text-xs text-destructive focus-visible:outline-ring"
            >
              <XCircleIcon aria-hidden="true" class="size-3.5"></XCircleIcon>Connection failed
            </button>
          </Tooltip.Control>{:else}{/if}{/if}
    </span>
  </div>
</SectionSurface>
