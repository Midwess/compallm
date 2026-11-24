<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Organization URL">
    <Fields.UrlPreview url="https://compallm.example/Research" onCopy={() => {}}></Fields.UrlPreview>
  </Sample><Sample label="Long URL">
    <Fields.UrlPreview url="https://compallm.example/long-organization-name-for-a-shared-workspace" onCopy={() => {}}
    ></Fields.UrlPreview>
  </Sample>
</Preview>
