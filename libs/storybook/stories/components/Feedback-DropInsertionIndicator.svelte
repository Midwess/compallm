<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Insertion position · 2 px"><Feedback.DropInsertionIndicator></Feedback.DropInsertionIndicator></Sample>
</Preview>
