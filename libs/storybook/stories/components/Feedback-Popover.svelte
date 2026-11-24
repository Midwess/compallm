<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import { CheckboxOption } from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Local display controls">
    <Feedback.Popover.Control label="Display options">
      <h2>Display options</h2>
      <CheckboxOption defaultChecked>Show descriptions</CheckboxOption><CheckboxOption>
        Show archived items
      </CheckboxOption><Feedback.HelperText>Changes apply to this sample only.</Feedback.HelperText>
    </Feedback.Popover.Control>
  </Sample>
</Preview>
