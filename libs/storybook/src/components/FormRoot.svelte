<script lang="ts" generics="T extends object">
  import { onMount, untrack } from 'svelte'
  import type { Snippet } from 'svelte'
  import type { HTMLFormAttributes } from 'svelte/elements'
  import type { FormModel } from './form-model.svelte'
  import { setFormContext } from '../context'
  let { model, children, onsubmit, onkeydown, ...props }: HTMLFormAttributes & { model: FormModel<T>; children?: Snippet } =
    $props()
  setFormContext(untrack(() => model) as unknown as FormModel<object>)
  onMount(() => model.mount())
</script>

<form
  {...props}
  novalidate
  onsubmit={(event) => {
    const cancelled = event.defaultPrevented
    onsubmit?.(event)
    const handled = event.defaultPrevented
    event.preventDefault()
    if (!cancelled && !handled) void model.submit()
  }}
  onreset={(event) => {
    event.preventDefault()
    model.reset()
  }}
  onkeydown={(event) => {
    onkeydown?.(event)
    if (event.defaultPrevented) return
    if (
      event.key === 'Enter' &&
      model.buttonOnly() &&
      !event.isComposing &&
      event.target instanceof HTMLElement &&
      !event.target.closest('button,textarea,[contenteditable=true]')
    )
      event.preventDefault()
  }}
>
  {@render children?.()}
</form>
