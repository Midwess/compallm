<script lang="ts">
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import * as Buttons from '../../src/components/buttons'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Circular icon actions">
    <Row>
      <Buttons.IconButton aria-label="Add"><PlusIcon aria-hidden="true"></PlusIcon></Buttons.IconButton><Buttons.IconButton
        aria-label="Unavailable add"
        disabled
      >
        <PlusIcon aria-hidden="true"></PlusIcon>
      </Buttons.IconButton>
    </Row>
  </Sample>
</Preview>
