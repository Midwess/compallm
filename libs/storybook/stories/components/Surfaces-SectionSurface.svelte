<script lang="ts">
  import { SectionSurface as Surface } from '../../src/components/surfaces'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Neutral section · 10 px corners" wide>
    <Surface>
      <p>Composable section content</p>
      <p class="text-[12px] leading-4.5 text-muted-foreground">Shared neutral background and token-based padding.</p>
    </Surface>
  </Sample>
</Preview>
