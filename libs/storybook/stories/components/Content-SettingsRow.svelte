<script lang="ts">
  import * as Content from '../../src/components/content'
  import { Switch } from '../../src/components/selection'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Settings row · 65 px" wide>
    <Content.SettingsRow description="Explain what this preference changes.">
      {#snippet label()}<label for="descriptions">Show descriptions</label>{/snippet}<Switch id="descriptions" defaultChecked
      ></Switch>
    </Content.SettingsRow>
  </Sample>
</Preview>
