<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Registration feedback"><Feedback.EmailRegistrationError></Feedback.EmailRegistrationError></Sample>
</Preview>
