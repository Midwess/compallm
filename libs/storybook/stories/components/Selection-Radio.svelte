<script lang="ts">
  import * as Selection from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Radio group">
    <fieldset class="flex max-w-full flex-wrap items-center gap-2">
      <legend class="text-[12px] leading-4.5 text-muted-foreground">Default response</legend><label
        class="inline-flex min-h-6 cursor-pointer items-center gap-0.5 text-xsm"
      >
        <Selection.Radio name="response" value="concise" defaultChecked></Selection.Radio>Concise
      </label><label class="inline-flex min-h-6 cursor-pointer items-center gap-0.5 text-xsm">
        <Selection.Radio name="response" value="detailed"></Selection.Radio>Detailed
      </label><Selection.Radio name="response" aria-label="Unavailable" disabled></Selection.Radio>
    </fieldset>
  </Sample>
</Preview>
