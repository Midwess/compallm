<script lang="ts">
  import ProviderIcon from './ProviderIcon.svelte'
  import { Popup } from './popup'
  import { fieldModel } from './form-context.svelte'
  import { cx } from './shared'
  let {
    name,
    provider,
    src,
    onFileChange,
    disabled = false,
    class: className,
  }: {
    name?: string
    provider?: string | null
    src?: string | null
    onFileChange?: (file: File | null) => void
    disabled?: boolean
    class?: string
  } = $props()
  const form = fieldModel(() => name)
  let input: HTMLInputElement
  const change = (file: File | null) => {
    if (form && name) form.change(name, file)
    else onFileChange?.(file)
  }
</script>

<input
  bind:this={input}
  type="file"
  accept="image/png,image/jpeg,image/webp"
  class="sr-only"
  tabindex="-1"
  aria-label="Provider avatar image"
  {disabled}
  onchange={(e) => {
    const file = e.currentTarget.files?.[0]
    if (file) change(file)
    e.currentTarget.value = ''
  }}
/><Popup.Root>
  <Popup.Trigger
    data-form-field={name}
    aria-label="Change provider avatar"
    {disabled}
    class={cx(
      'size-8 shrink-0 overflow-hidden rounded-lg bg-elevated text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50',
      className,
    )}
  >
    <ProviderIcon {provider} logoPath={src ?? undefined} class="size-5" />
  </Popup.Trigger><Popup.Content>
    <Popup.Item onSelect={() => input.click()}>Upload image</Popup.Item><Popup.Item onSelect={() => change(null)}>
      Use provider logo
    </Popup.Item>{#if src}<Popup.Item onSelect={() => change(null)}>Remove image</Popup.Item>{/if}
  </Popup.Content>
</Popup.Root>
