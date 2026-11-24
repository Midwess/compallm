<script lang="ts">
  import type { Snippet } from 'svelte'
  let { actor, children, dateTime, timeLabel }: { actor: string; children?: Snippet; dateTime: string; timeLabel: string } =
    $props()
</script>

<p
  class="flex flex-wrap items-center gap-0.5 text-ui-12 text-muted-foreground [&>strong]:font-medium [&>strong]:text-foreground"
>
  <strong>{actor}</strong><span>{@render children?.()}</span><span aria-hidden="true">·</span><time datetime={dateTime}>
    {timeLabel}
  </time>
</p>
