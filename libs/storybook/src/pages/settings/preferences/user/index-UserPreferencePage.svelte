<script lang="ts">
  import { LoadingRegion } from '../../../../components/loading'
  import { Form } from '../../../../components/form'
  import { Button } from '../../../../components/buttons'
  import { SettingsContent, Empty } from '../../shared'
  import { configureUserPreferenceForm } from './form'
  import { untrack } from 'svelte'
  import type { Content } from '../../../../components/shared'
  import { readStore } from '../../../page-state.svelte'
  import { getUserPreferenceController, getSettingsController, setCodeAppearance } from '../../../../context'
  import { resolveCodeAppearance } from '../../../../components/code-appearance'
  import { systemLight } from '../../../page-state.svelte'
  import ContentValue from '../../../../components/Content.svelte'

  let {
    children,
  }: {
    children: Content
  } = $props()
  const __preferences = getUserPreferenceController()
  const controller = untrack(() => getUserPreferenceController())
  const settings = untrack(() => getSettingsController())
  setCodeAppearance(() => {
    const draft = controller.store.getState().draft
    return resolveCodeAppearance(draft.codeTheme, draft.theme, draft.codeFont, draft.codeFontSize, systemLight())
  })
  let saved = $derived(readStore(__preferences.store, (state) => state.saved))
  let ready = $derived(!!saved)
  let status = $derived(readStore(__preferences.store, (state) => state.status))
  $effect(() => {
    void [controller, settings, ready]
    return untrack(() => (ready ? configureUserPreferenceForm(controller, settings.apiError) : undefined))
  })
</script>

{#if status === 'denied' || (status === 'error' && !saved)}<SettingsContent>
    <Empty title={status === 'denied' ? 'Preferences are unavailable' : 'Could not load preferences'}>
      {#if status === 'denied'}Your session cannot access these preferences.{:else}{#if status === 'error'}Try loading your
          saved preferences again.{:else}Loading your saved preferences.{/if}{/if}
    </Empty>{#if status === 'error' || status === 'denied'}<Button variant="ghost" onclick={() => void controller.load()}>
        Retry
      </Button>{:else}{/if}
  </SettingsContent>{:else}<LoadingRegion
    ready={!!saved}
    pending={status === 'loading' || status === 'idle'}
    label="Loading preferences"
  >
    <Form.Root model={controller.store.form}><ContentValue value={children} /></Form.Root>
  </LoadingRegion>{/if}
