<script lang="ts">
  import MagnifyingGlassIcon from 'phosphor-svelte/lib/MagnifyingGlassIcon'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import HardDrivesIcon from 'phosphor-svelte/lib/HardDrivesIcon'
  import ChatCircleIcon from 'phosphor-svelte/lib/ChatCircleIcon'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  {#each [12, 14, 16, 20] as const as size (size)}<Sample label={`Phosphor Regular · ${size} px`}>
      <Row>
        <MagnifyingGlassIcon {size} aria-label="Search"></MagnifyingGlassIcon><CaretDownIcon {size} aria-label="Expand"
        ></CaretDownIcon><PlusIcon {size} aria-label="Add"></PlusIcon><HardDrivesIcon {size} aria-label="Providers"
        ></HardDrivesIcon><ChatCircleIcon {size} aria-label="Chat"></ChatCircleIcon>
      </Row>
    </Sample>{/each}
</Preview>
