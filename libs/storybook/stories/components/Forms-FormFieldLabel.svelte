<script lang="ts">
  import * as Forms from '../../src/components/forms'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Persistent label">
    <Forms.FormFieldLabel for="label-sample">Field label</Forms.FormFieldLabel><input
      class={`
  block h-8 w-full min-w-0 rounded-lg border-(length:--border-hairline) border-input bg-inset px-1.5
  py-0.75 text-[14px] leading-5 text-foreground placeholder:text-muted-foreground placeholder:opacity-100
  focus:outline-1 focus:outline-offset-1 focus:outline-ring aria-invalid:border-destructive
  disabled:cursor-not-allowed disabled:opacity-50
`}
      id="label-sample"
    />
  </Sample>
</Preview>
