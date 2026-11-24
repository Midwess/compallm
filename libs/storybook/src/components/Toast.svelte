<script lang="ts">
  import XIcon from 'phosphor-svelte/lib/XIcon'
  import InfoIcon from 'phosphor-svelte/lib/InfoIcon'
  import WarningCircleIcon from 'phosphor-svelte/lib/WarningCircleIcon'
  import WarningIcon from 'phosphor-svelte/lib/WarningIcon'
  import CheckCircleIcon from 'phosphor-svelte/lib/CheckCircleIcon'
  import SpinnerGapIcon from 'phosphor-svelte/lib/SpinnerGapIcon'
  import RichMessage from './RichMessage.svelte'
  import { cx } from './shared'
  import { loadingMotion, interaction } from './motion'
  let {
    message,
    title,
    tone = 'info',
    variant,
    action,
    onDismiss,
    dismissLabel = 'Dismiss notification',
    class: className,
  }: {
    message: string
    title?: string
    tone?: 'info' | 'error' | 'warning' | 'success' | 'loading'
    variant?: 'info' | 'error' | 'warning' | 'success' | 'loading'
    action?: { label: string; onClick: () => void }
    onDismiss: () => void
    dismissLabel?: string
    class?: string
  } = $props()
  const tones = {
    info: { Icon: InfoIcon, color: 'text-primary', label: 'Info' },
    error: { Icon: WarningCircleIcon, color: 'text-destructive', label: 'Error' },
    warning: { Icon: WarningIcon, color: 'text-warning', label: 'Warning' },
    success: { Icon: CheckCircleIcon, color: 'text-success', label: 'Success' },
    loading: { Icon: SpinnerGapIcon, color: 'text-primary', label: 'Loading' },
  }
  let current = $derived(variant ?? tone)
  let appearance = $derived(tones[current])
  let Icon = $derived(appearance.Icon)
</script>

<div
  data-ui="toast"
  data-tone={current}
  class={cx(
    'flex w-fit min-w-0 max-w-100 items-start gap-2.5 rounded-xl border border-border bg-elevated p-2 font-sans text-xsm leading-5 text-foreground shadow-floating',
    className,
  )}
>
  <span class={cx('mt-0.5 inline-flex shrink-0', appearance.color)}>
    {#if current === 'loading'}<span use:loadingMotion><Icon size={20} weight="regular" aria-hidden="true" /></span>{:else}<Icon
        size={20}
        weight="regular"
        aria-hidden="true"
      />{/if}
  </span>
  <div class="min-w-0 flex-1">
    <div class="wrap-anywhere" role={current === 'error' ? 'alert' : 'status'} aria-atomic="true">
      <span class="sr-only">{appearance.label}: </span>{#if title}<p class="text-sm leading-5.5 font-semibold">{title}</p>{/if}
      <div class={cx(title ? 'mt-0.5 text-muted-foreground' : 'text-foreground')}><RichMessage {message} /></div>
    </div>
    <div class="mt-1 flex flex-wrap items-center justify-end gap-x-3 gap-y-1">
      <button
        type="button"
        onclick={onDismiss}
        class="min-h-6 cursor-pointer rounded text-muted-foreground hover:text-foreground focus-visible:outline-2 focus-visible:outline-ring"
        use:interaction
      >
        Dismiss
      </button>{#if action}<button
          type="button"
          onclick={action.onClick}
          class="min-h-6 cursor-pointer rounded text-primary hover:text-ring focus-visible:outline-2 focus-visible:outline-ring"
          use:interaction
        >
          {action.label}
        </button>{/if}
    </div>
  </div>
  <button
    type="button"
    onclick={onDismiss}
    aria-label={dismissLabel}
    class="inline-flex size-6 shrink-0 cursor-pointer items-center justify-center rounded text-muted-foreground hover:text-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
    use:interaction
  >
    <XIcon size={16} weight="regular" aria-hidden="true" />
  </button>
</div>
