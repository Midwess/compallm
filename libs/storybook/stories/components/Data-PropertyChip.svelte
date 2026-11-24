<script lang="ts">
  import CalendarIcon from 'phosphor-svelte/lib/CalendarIcon'
  import FlagIcon from 'phosphor-svelte/lib/FlagIcon'
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Property actions">
    <Row>
      <Data.PropertyChip>
        {#snippet icon()}<CalendarIcon size={14} aria-hidden="true"></CalendarIcon>{/snippet}Target date
      </Data.PropertyChip><Data.PropertyChip>
        {#snippet icon()}<FlagIcon size={14} aria-hidden="true"></FlagIcon>{/snippet}No priority
      </Data.PropertyChip>
    </Row>
  </Sample>
</Preview>
