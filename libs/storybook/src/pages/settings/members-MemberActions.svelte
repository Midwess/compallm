<script lang="ts">
  import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon'

  import { Popup } from '../../components/popup'

  import type { SettingsMember } from './types'

  let {
    member,
  }: {
    member: SettingsMember
  } = $props()
</script>

<Popup.Root>
  <Popup.Trigger aria-label={`Actions for ${member.name}`} class="size-8 p-0">
    <DotsThreeIcon class="size-4 text-muted-foreground" aria-hidden="true"></DotsThreeIcon>
  </Popup.Trigger><Popup.Content aria-label={`Actions for ${member.name}`} class="bg-elevated">
    <Popup.Item>
      {#snippet child({ props })}<a {...props} href={`mailto:${encodeURIComponent(member.email)}`}> Send email </a>{/snippet}
    </Popup.Item>
  </Popup.Content>
</Popup.Root>
