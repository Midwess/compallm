<script lang="ts">
  import { FormError } from '../../../components/form'

  import { Button } from '../../../components/buttons'

  import { CodeEditor } from '../../../components/code-block'
  import { SectionSurface } from '../../../components/surfaces'

  import { DisabledTooltip } from '../../../components/feedback'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  import { ownerExplanation } from './index'

  let {
    locked,
    owner,
  }: {
    locked: boolean
    owner: boolean
  } = $props()
  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController().pages.role)
  const shared = untrack(() => getSettingsController())
  let draft = $derived(
    readStore(
      __settings.pages.role.store,
      (s) => (s.draft ? { parsing: s.draft.parsing, diagnostics: s.draft.diagnostics } : null),
      ['draft.parsing', 'draft.diagnostics'],
    ),
  )
</script>

{#if !draft}{:else}{@const invalid = draft.parsing || draft.diagnostics.length > 0}<SectionSurface
    class="flex min-w-0 w-full flex-col p-2"
  >
    <div class="flex shrink-0 flex-wrap items-center justify-end gap-2 px-2 pb-2 text-xs">
      <DisabledTooltip disabled={owner} reason={ownerExplanation}>
        <Button variant="ghost" class="h-6 px-2 text-xs" disabled={invalid || locked} onclick={() => c.formatPolicy()}>
          Format
        </Button>
      </DisabledTooltip><Button
        variant="ghost"
        class="h-6 px-2 text-xs"
        onclick={() => shared.pages.dialogs.copy(c.store.form.get('draft.source') as string)}
      >
        Copy
      </Button><span class="rounded-lg bg-elevated px-2.5 py-1 font-sans text-xs leading-4 text-foreground">Cedar</span>
    </div>
    <DisabledTooltip disabled={owner} reason={ownerExplanation} allowInteraction>
      <CodeEditor
        label="Cedar policy source"
        language="cedar"
        showToolbar={false}
        invalid={draft.diagnostics.length > 0}
        readOnly={locked}
        name="draft.source"
      ></CodeEditor><FormError name="draft.source"></FormError>
    </DisabledTooltip>
  </SectionSurface>{/if}
