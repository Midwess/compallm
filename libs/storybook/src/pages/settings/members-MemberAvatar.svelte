<script lang="ts">
  import { Avatar } from '../../components/content'

  import type { SettingsMember } from './types'

  let {
    member,
  }: {
    member: SettingsMember
  } = $props()

  let tone = $derived(Array.from(member.name).reduce((sum, letter, index) => sum + letter.charCodeAt(0) * (index + 1), 0) % 3)
  let colors = $derived([
    '[&>span]:bg-[color-mix(in_oklab,var(--elevated)_75%,var(--success))]',
    '[&>span]:bg-[color-mix(in_oklab,var(--elevated)_75%,var(--primary))]',
    '[&>span]:bg-[color-mix(in_oklab,var(--elevated)_75%,var(--muted-foreground))]',
  ])
</script>

<span class={`${colors[tone]} inline-flex shrink-0 [&>span]:text-[9px] [&>span]:font-normal`}>
  <Avatar name={member.name} size={24}></Avatar>
</span>
