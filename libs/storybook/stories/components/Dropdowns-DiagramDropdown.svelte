<script lang="ts">
  import * as Menus from '../../src/components/dropdowns'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview><Sample label="Diagram style"><Menus.DiagramDropdown value="Hairline"></Menus.DiagramDropdown></Sample></Preview>
