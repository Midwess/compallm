<script lang="ts">
  import * as Feedback from '../../src/components/feedback'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Contextual field help" wide>
    <span class="inline-flex items-center gap-1 text-sm font-medium">
      Thinking efforts <Feedback.FieldHelp label="About thinking efforts">
        Map Low → Max to the thinking-effort values your model accepts (for example, low, medium, high). Higher effort can use
        more time and tokens. Fill levels in order; leave unused levels empty.
      </Feedback.FieldHelp>
    </span>
  </Sample>
</Preview>
