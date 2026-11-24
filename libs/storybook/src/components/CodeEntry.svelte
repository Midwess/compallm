<script lang="ts">
  import { untrack } from 'svelte'
  import { otpLength, otpPattern, otpCharacters } from '@compallm/contract/auth'
  import { fieldModel } from './form-context.svelte'
  import TextField from './TextField.svelte'
  let {
    name,
    value = $bindable(''),
    defaultValue = '',
    onValueChange,
    disabled = false,
    'aria-label': label,
    ...a11y
  }: {
    name?: string
    value?: string
    defaultValue?: string
    onValueChange?: (value: string) => void
    disabled?: boolean
    'aria-label': string
    'aria-invalid'?: boolean
    'aria-describedby'?: string
  } = $props()
  const form = fieldModel(() => name)
  const inputs: HTMLInputElement[] = []
  untrack(() => {
    if (!value) value = defaultValue
  })
  let current = $derived(
    String(form && name ? (form.get(name) ?? '') : value)
      .padEnd(otpLength, ' ')
      .slice(0, otpLength),
  )
  function update(code: string[], focus: number) {
    const next = code.join('').trimEnd()
    if (form && name) form.change(name, next)
    else {
      value = next
      onValueChange?.(next)
    }
    inputs[Math.max(0, Math.min(otpLength - 1, focus))]?.focus()
  }
  function insert(text: string, index: number) {
    const digits = otpCharacters(text)
    if (!digits) return
    const start = digits.length === otpLength ? 0 : index
    const next = current.split('')
    for (let offset = 0; offset < digits.length && start + offset < otpLength; offset++) next[start + offset] = digits[offset]
    update(next, start + digits.length)
  }
</script>

<div
  role="group"
  aria-label={label}
  data-ui="code-entry"
  data-form-field={name}
  class="flex w-88 max-w-full gap-1 [&_input]:min-w-0 [&_input]:max-w-13 [&_input]:flex-1"
  onfocusout={(e) => {
    if (!e.currentTarget.contains(e.relatedTarget as Node) && name) form?.blur(name)
  }}
>
  {#each Array(otpLength) as _, index (index)}<TextField
      {...a11y}
      aria-invalid={form && name ? form.invalid(name) || undefined : a11y['aria-invalid']}
      aria-describedby={a11y['aria-describedby'] ?? (form && name ? `${name}-error` : undefined)}
      bind:element={inputs[index]}
      value={current[index].trim()}
      class="aspect-square h-auto w-13 p-0 text-center text-ui-20"
      inputmode="numeric"
      pattern={otpPattern}
      maxlength={otpLength}
      {disabled}
      autocomplete={index === 0 ? 'one-time-code' : 'off'}
      aria-label={`Digit ${index + 1} of ${otpLength}`}
      onfocus={(e) => e.currentTarget.select()}
      oninput={(e) => {
        if (e.currentTarget.value) insert(e.currentTarget.value, index)
        else {
          const next = current.split('')
          next[index] = ' '
          update(next, index)
        }
      }}
      onpaste={(e) => {
        e.preventDefault()
        insert(e.clipboardData?.getData('text') ?? '', index)
      }}
      onkeydown={(e) => {
        if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
          e.preventDefault()
          inputs[index + (e.key === 'ArrowLeft' ? -1 : 1)]?.focus()
        }
        if (e.key === 'Backspace' && current[index] === ' ' && index > 0) {
          e.preventDefault()
          const next = current.split('')
          next[index - 1] = ' '
          update(next, index - 1)
        }
      }}
    />{/each}
</div>
