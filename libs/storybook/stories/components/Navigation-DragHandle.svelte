<script lang="ts">
  import * as Navigation from '../../src/components/navigation'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Handle · caller owns reordering">
    <Row>
      <Navigation.DragHandle label="Reorder item"></Navigation.DragHandle><Navigation.DragHandle
        label="Reorder unavailable"
        disabled
      ></Navigation.DragHandle>
    </Row>
  </Sample>
</Preview>
