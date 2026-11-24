<script lang="ts">
  import { untrack } from 'svelte'

  import ContentValue from '../../components/Content.svelte'

  let {
    value,
  }: {
    value?: number | null
  } = $props()

  let now = $state(untrack(() => Date.now()))
  let age = $derived(value ? Math.max(0, Math.floor(now / 1000) - value) : 86400)
  let recent = $derived(age < 86400)
  $effect(() => {
    void [value, recent]
    return untrack(() => {
      if (!recent) return
      const timer = setInterval(() => (now = Date.now()), 60000)
      return () => clearInterval(timer)
    })
  })
</script>

{#if !value}<span class="text-muted-foreground">—</span>{:else}{@const date = new Date(value * 1000)}{@const sameYear =
    date.getUTCFullYear() === new Date(now).getUTCFullYear()}{@const relative = new Intl.RelativeTimeFormat(undefined, {
    numeric: 'always',
  })}{@const label =
    age < 60
      ? 'Just now'
      : age < 3600
        ? relative.format(-Math.floor(age / 60), 'minute')
        : age < 86400
          ? relative.format(-Math.floor(age / 3600), 'hour')
          : date.toLocaleDateString(
              undefined,
              sameYear ? { month: 'short', day: 'numeric' } : { month: 'short', year: 'numeric' },
            )}<time
    datetime={date.toISOString()}
    title={date.toLocaleString()}
    class="whitespace-nowrap text-xs text-muted-foreground"
  >
    <ContentValue value={label} />
  </time>{/if}
