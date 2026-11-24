<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Empty state" wide>
    <Feedback.EmptyState title="No items yet">New items will appear here when they are available.</Feedback.EmptyState>
  </Sample>
</Preview>
