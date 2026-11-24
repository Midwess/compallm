<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Inherited prefix · editable path" wide>
    <Fields.PrefixedTextField aria-label="Endpoint path" value="/chat/completions">
      {#snippet prefix()}<span class="px-2 font-mono text-ui-11">{'{{base_url}}'}</span>{/snippet}
    </Fields.PrefixedTextField>
  </Sample><Sample label="Disabled prefix field" wide>
    <Fields.PrefixedTextField aria-label="Disabled endpoint path" value="/messages" disabled>
      {#snippet prefix()}<span class="px-2 font-mono text-ui-11">{'{{base_url}}'}</span>{/snippet}
    </Fields.PrefixedTextField>
  </Sample>
</Preview>
