<script lang="ts">
  import WarningIcon from 'phosphor-svelte/lib/WarningIcon'
  import { Tooltip as UI } from 'bits-ui'
  import { attachFieldError } from './field-error'
  import { fieldErrorMotion } from './motion'
  let {
    anchor,
    message,
    id,
    validationVersion = 0,
  }: { anchor: HTMLElement; message: string; id: string; validationVersion?: number } = $props()
  let badge = $state<HTMLButtonElement | null>(null)
  let content = $state<HTMLElement | null>(null)
  let visible = $state(true)
  let expanded = $state(true)
  let interacting = $state(false)
  let dismissed = $state(false)
  let open = $derived(visible && !dismissed && (expanded || interacting))
  let container = $derived(anchor.closest('dialog') ?? anchor.ownerDocument.body)
  $effect(() => {
    void validationVersion
    expanded = true
    dismissed = false
    const timer = setTimeout(() => (expanded = false), 5000)
    return () => clearTimeout(timer)
  })
  $effect(() =>
    attachFieldError(
      anchor,
      id,
      () => badge,
      (value) => (visible = value),
    ),
  )
  $effect(() => {
    if (!content) return
    return fieldErrorMotion(content).destroy
  })
  function dismiss() {
    expanded = false
    interacting = false
    dismissed = true
  }
</script>

<UI.Portal to={container}>
  <span {id} class="sr-only" role="alert">{message}</span>
  <UI.Provider delayDuration={0}>
    <UI.Root
      {open}
      onOpenChange={(next) => {
        if (!next) interacting = false
      }}
    >
      <UI.Trigger
        bind:ref={badge}
        aria-label="Show validation error"
        aria-describedby={id}
        class="fixed z-50 inline-flex size-5 cursor-help items-center justify-center rounded-full bg-error-background text-error-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-destructive"
        onpointerenter={() => {
          interacting = true
          dismissed = false
        }}
        onpointerleave={() => (interacting = false)}
        onfocus={() => {
          interacting = true
          dismissed = false
        }}
        onblur={() => (interacting = false)}
        onclick={dismiss}
        onkeydown={(event) => {
          if (event.key === 'Escape') {
            event.preventDefault()
            event.stopPropagation()
            dismiss()
          }
        }}
      >
        <WarningIcon weight="fill" aria-hidden="true" class="size-3" />
      </UI.Trigger>
      {#if open}<UI.Portal to={container}>
          <UI.Content
            bind:ref={content}
            customAnchor={badge}
            role="tooltip"
            side="top"
            align="start"
            sideOffset={8}
            collisionPadding={8}
            strategy="fixed"
            onInteractOutside={(event) => event.preventDefault()}
            onEscapeKeydown={(event) => {
              event.preventDefault()
              event.stopPropagation()
              dismiss()
            }}
            class="z-50 min-h-5 max-w-xs origin-bottom-left rounded-full bg-error-background px-2 py-1 font-sans text-xs leading-3 text-error-foreground shadow-floating data-[side=bottom]:origin-top-left"
          >
            {message}
          </UI.Content>
        </UI.Portal>{/if}
    </UI.Root>
  </UI.Provider>
</UI.Portal>
