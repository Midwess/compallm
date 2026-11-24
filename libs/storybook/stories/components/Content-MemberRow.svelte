<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Member identity" wide>
    <Content.MemberRow name="Alex Morgan" email="alex@example.com" role="Member"></Content.MemberRow>
  </Sample>
</Preview>
