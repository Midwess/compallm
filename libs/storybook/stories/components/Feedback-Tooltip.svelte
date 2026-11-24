<script lang="ts">
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import * as Feedback from '../../src/components/feedback'
  import { Button } from '../../src/components/buttons'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Hover or keyboard focus">
    <Feedback.Tooltip.Control content="Add provider">
      <Button aria-label="Add provider"><PlusIcon size={14} aria-hidden="true"></PlusIcon></Button>
    </Feedback.Tooltip.Control>
  </Sample>
</Preview>
