<script lang="ts">
  import * as Buttons from '../../src/components/buttons'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Ready"><Buttons.ResendPrompt></Buttons.ResendPrompt></Sample><Sample
    label="Cooldown · caller supplies remaining time"
  >
    <Buttons.ResendPrompt remaining={28}></Buttons.ResendPrompt>
  </Sample>
</Preview>
