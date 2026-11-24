<script lang="ts" generics="T extends object">
  import { onMount, untrack } from 'svelte'
  import type { Snippet } from 'svelte'
  import { currentForm } from './form-context.svelte'
  import { setFormContext } from '../context'
  import type { FormOptions } from './form-model.svelte'
  let { names, options = {}, children }: { names: string[]; options?: FormOptions<T>; children?: Snippet } = $props()
  const parent = currentForm()
  if (!parent) throw new Error('Form.Scope requires Form.Root')
  const model = untrack(() => parent.scope(names, options as unknown as FormOptions<object>))
  setFormContext(model)
  onMount(model.mount)
</script>

{@render children?.()}
