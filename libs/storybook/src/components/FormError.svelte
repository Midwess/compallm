<script lang="ts">
  import { onMount } from 'svelte'
  import { currentForm } from './form-context.svelte'
  import FieldErrorIndicator from './FieldErrorIndicator.svelte'
  let { name, id, anchor }: { name: string; id?: string; anchor?: HTMLElement } = $props()
  const form = currentForm()
  let marker: HTMLSpanElement
  let target = $state<HTMLElement | null>(null)
  let message = $state<string>()
  let revision = $state(0)
  onMount(() => {
    const update = () => {
      message = form?.message(name)
      revision = form?.validationVersion(name) ?? 0
      const root = marker.closest('form') ?? marker.parentElement
      const named =
        anchor ?? root?.querySelector<HTMLElement>(`[data-form-field="${CSS.escape(name)}"], [name="${CSS.escape(name)}"]`)
      target = named?.matches('input[type=file]') ? (named.closest('label') ?? named) : (named ?? null)
    }
    update()
    return form?.subscribe(update, name)
  })
</script>

<span bind:this={marker} hidden></span>{#if message && target}<FieldErrorIndicator
    anchor={target}
    {message}
    id={id ?? `${name}-error`}
    validationVersion={revision}
  />{/if}
