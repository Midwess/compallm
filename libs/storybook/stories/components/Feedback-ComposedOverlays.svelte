<script lang="ts">
  import * as Feedback from '../../src/components/feedback'
  import { Button } from '../../src/components/buttons'

  import Preview from './Preview.svelte'
</script>

<Preview>
  <Feedback.Tooltip.Root>
    <Feedback.Tooltip.Trigger>
      {#snippet child({ props })}<Button {...props}>Hover or focus</Button>{/snippet}
    </Feedback.Tooltip.Trigger><Feedback.Tooltip.Content>Composed tooltip</Feedback.Tooltip.Content>
  </Feedback.Tooltip.Root><Feedback.Popover.Root>
    <Feedback.Popover.Trigger>
      {#snippet child({ props })}<Button {...props}>Composed popover</Button>{/snippet}
    </Feedback.Popover.Trigger><Feedback.Popover.Content>
      <Feedback.HelperText>Outside clicks and Escape dismiss this popover.</Feedback.HelperText>
    </Feedback.Popover.Content>
  </Feedback.Popover.Root>
</Preview>
