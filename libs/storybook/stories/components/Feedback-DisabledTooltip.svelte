<script lang="ts">
  import * as Feedback from '../../src/components/feedback'
  import { Button } from '../../src/components/buttons'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Disabled control in a clipped container">
    <div class="overflow-hidden rounded-md border border-border p-2">
      <Feedback.DisabledTooltip disabled reason="The Owner role cannot be edited or deleted.">
        <Button disabled>Delete role</Button>
      </Feedback.DisabledTooltip>
    </div>
  </Sample>
</Preview>
