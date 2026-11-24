<script lang="ts">
  import * as Selection from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Provider choices">
    <Selection.ChoiceCard name="provider" value="first" description="Provider models" defaultChecked>
      Provider one
    </Selection.ChoiceCard><Selection.ChoiceCard name="provider" value="second" description="Provider models">
      Provider two
    </Selection.ChoiceCard>
  </Sample>
</Preview>
