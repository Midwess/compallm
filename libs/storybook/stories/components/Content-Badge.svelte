<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Status tones">
    <Row>
      <Content.Badge>Member</Content.Badge><Content.Badge tone="success">Active</Content.Badge><Content.Badge tone="warning">
        Pending
      </Content.Badge><Content.Badge tone="error">Failed</Content.Badge>
    </Row>
  </Sample>
</Preview>
