<script lang="ts">
  import { untrack } from 'svelte'

  import { sessionDateFormatter, sessionYearFormatter, sessionTimeFormatter } from './account'
  import ContentValue from '../../components/Content.svelte'

  let {
    label,
    timestamp,
  }: {
    label: string
    timestamp: string
  } = $props()

  let now = $state(untrack(() => Date.now()))
  let seconds = $derived(Number(timestamp))
  let age = $derived(Math.max(0, Math.floor(now / 1000) - seconds))
  let recent = $derived(age < 86400)
  $effect(() => {
    void [timestamp, recent]
    return untrack(() => {
      if (!recent) return
      const timer = setInterval(() => (now = Date.now()), 60000)
      return () => clearInterval(timer)
    })
  })
  const date = untrack(() => new Date(seconds * 1000))
</script>

{#if !Number.isFinite(date.getTime())}{:else}{@const text =
    age < 60
      ? 'Just now'
      : age < 3600
        ? `${Math.floor(age / 60)}m ago`
        : recent
          ? `${Math.floor(age / 3600)}h ago`
          : (date.getFullYear() === new Date(now).getFullYear() ? sessionDateFormatter : sessionYearFormatter).format(
              date,
            )}<span
    class="inline-flex items-center gap-1.5 whitespace-nowrap"
    title={`${label}: ${sessionTimeFormatter.format(date)}`}
  >
    <span><ContentValue value={label} /></span><time datetime={date.toISOString()}>{text}</time>
  </span>{/if}
