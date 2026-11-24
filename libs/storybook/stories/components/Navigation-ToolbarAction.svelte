<script lang="ts">
  import FunnelIcon from 'phosphor-svelte/lib/FunnelIcon'
  import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon'
  import StarIcon from 'phosphor-svelte/lib/StarIcon'
  import BellIcon from 'phosphor-svelte/lib/BellIcon'
  import CopyIcon from 'phosphor-svelte/lib/CopyIcon'
  import * as Navigation from '../../src/components/navigation'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Toolbar controls">
    <Row>
      <Navigation.ToolbarAction label="Filter">
        <FunnelIcon size={14} aria-hidden="true"></FunnelIcon>
      </Navigation.ToolbarAction><Navigation.ToolbarAction label="More">
        <DotsThreeIcon size={14} aria-hidden="true"></DotsThreeIcon>
      </Navigation.ToolbarAction><Navigation.ToolbarAction label="Favorite">
        <StarIcon size={14} aria-hidden="true"></StarIcon>
      </Navigation.ToolbarAction><Navigation.ToolbarAction label="Notifications">
        <BellIcon size={14} aria-hidden="true"></BellIcon>
      </Navigation.ToolbarAction><Navigation.ToolbarAction label="Copy URL">
        <CopyIcon size={14} aria-hidden="true"></CopyIcon>
      </Navigation.ToolbarAction>
    </Row>
  </Sample>
</Preview>
