<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
  const avatarImage =
    "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='40' height='40'%3E" +
    "%3Crect width='40' height='40' fill='%237c3aed'/%3E%3C/svg%3E"
</script>

<Preview>
  <Sample label="Initials">
    <Row>
      <Content.Avatar name="Alex Morgan"></Content.Avatar><Content.Avatar name="Sam Lee" size={24}
      ></Content.Avatar><Content.Avatar name="Failed Image" src="/missing-avatar.png"></Content.Avatar><Content.Avatar
        name="Loaded Image"
        src={avatarImage}
      ></Content.Avatar>
    </Row>
  </Sample>
</Preview>
