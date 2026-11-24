<script lang="ts">
  import * as Feedback from '../../src/components/feedback'
  import { Button } from '../../src/components/buttons'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Contextual feedback" wide>
    <Feedback.Callout title="Access is disabled">
      {#snippet action()}<Button variant="link">Configure →</Button>{/snippet}A workspace admin can configure access.
    </Feedback.Callout>
  </Sample>
</Preview>
