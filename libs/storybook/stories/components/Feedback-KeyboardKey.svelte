<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Keyboard hint">
    <Feedback.KeyboardKey>⌥</Feedback.KeyboardKey><Feedback.KeyboardKey>⌘ K</Feedback.KeyboardKey>
  </Sample>
</Preview>
