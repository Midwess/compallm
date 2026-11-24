<script lang="ts">
  import { useTransition } from '../../../components/motion'
  import { Button } from '../../../components/buttons'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  import MotionSurface from '../../../components/MotionSurface.svelte'

  const __settings = getSettingsController()
  let timing = $derived(useTransition('panel'))
  const c = untrack(() => getSettingsController().pages.role)
  let mode = $derived(readStore(__settings.pages.role.store, (s) => s.draft?.mode, ['draft.mode']))
  let invalid = $derived(
    readStore(__settings.pages.role.store, (s) => Boolean(s.draft?.parsing || s.draft?.diagnostics.length), [
      'draft.parsing',
      'draft.diagnostics.length',
    ]),
  )
</script>

<div
  role="group"
  aria-label="Policy editing mode"
  class="relative flex h-7 w-28 items-center gap-0.5 rounded-full bg-inset p-0.5"
>
  <MotionSurface
    initial={false}
    animate={{ x: mode === 'cedar' ? 'calc(100% + 2px)' : 0 }}
    transition={timing}
    aria-hidden="true"
    class="pointer-events-none absolute inset-y-0.5 left-0.5 w-[calc((100%-0.375rem)/2)]
rounded-full bg-elevated shadow-control"
  ></MotionSurface>{#each ['visual', 'cedar'] as const as value (value)}<Button
      aria-pressed={mode === value}
      variant="ghost"
      class="relative z-10 h-6 min-h-0 min-w-0 flex-1 rounded-full px-1 text-xs leading-4.5"
      disabled={value === 'visual' && mode === 'cedar' && invalid}
      onclick={() => c.setPolicyMode(value)}
    >
      {#if value === 'visual'}Visual{:else}Cedar{/if}
    </Button>{/each}
</div>
