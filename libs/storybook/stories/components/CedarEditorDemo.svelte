<script lang="ts">
  import { untrack } from 'svelte'
  import { CodeEditor } from '../../src'
  import { modelOperatorSource } from './code-fixtures'
  let { scenario = 'EditableCedar' }: { scenario?: string } = $props()
  let value = $state(
    untrack(() =>
      scenario === 'ShortCedar'
        ? 'permit(principal, action, resource);'
        : scenario === 'LongCedar'
          ? Array.from({ length: 8 }, () => modelOperatorSource).join('\n\n')
          : modelOperatorSource,
    ),
  )
</script>

<div class="max-w-208"><CodeEditor label="Editable Cedar" language="cedar" bind:value /></div>
