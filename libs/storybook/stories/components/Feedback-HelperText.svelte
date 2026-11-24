<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Supporting text"><Feedback.HelperText>Supporting information for this field.</Feedback.HelperText></Sample>
</Preview>
