<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Inline feedback">
    <Feedback.InlineError>This value needs attention. Check it and try again.</Feedback.InlineError>
  </Sample>
</Preview>
