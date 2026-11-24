<script lang="ts">
  import EyeIcon from 'phosphor-svelte/lib/EyeIcon'
  import EyeSlashIcon from 'phosphor-svelte/lib/EyeSlashIcon'

  import { Button } from '../../components/buttons'

  let {
    email,
    name,
  }: {
    email: string
    name: string
  } = $props()

  let visible = $state(false)
  let Icon = $derived(visible ? EyeSlashIcon : EyeIcon)
</script>

<div class="flex min-w-0 items-center justify-start gap-1.5 text-left">
  <span class="line-clamp-2 min-w-0 break-all whitespace-normal text-left">
    {#if visible}{email}{:else}••••••••{/if}
  </span><Button
    variant="ghost"
    class="size-6 shrink-0 p-0 text-muted-foreground"
    aria-label={`${visible ? 'Hide' : 'Show'} email for ${name}`}
    aria-pressed={visible}
    onclick={() => (visible = ((value) => !value)(visible))}
  >
    <Icon class="size-3" aria-hidden="true"></Icon>
  </Button>
</div>
