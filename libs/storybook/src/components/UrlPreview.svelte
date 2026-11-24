<script lang="ts">
  import TextLink from './TextLink.svelte'
  import IconButton from './IconButton.svelte'
  import LinkIcon from 'phosphor-svelte/lib/LinkIcon'
  import CopyIcon from 'phosphor-svelte/lib/CopyIcon'
  import { cx } from './shared'
  let { url, onCopy, class: className }: { url: string; onCopy: (url: string) => void; class?: string } = $props()
</script>

<div class={cx('flex h-8 min-w-0 items-center gap-1', className)}>
  <TextLink
    href={url || undefined}
    target="_blank"
    rel="noopener noreferrer"
    title={url}
    aria-label={url}
    class="flex h-full min-w-0 flex-1 items-center rounded-lg bg-inset text-xsm underline"
  >
    <LinkIcon aria-hidden="true" class="mr-1 size-3 shrink-0" /><span aria-hidden="true" class="min-w-0 truncate">
      {url.slice(0, Math.ceil(url.length / 2))}
    </span><span aria-hidden="true" class="shrink-0">{url.slice(Math.ceil(url.length / 2))}</span>
  </TextLink><IconButton aria-label="Copy URL" title="Copy URL" disabled={!url} onclick={() => onCopy(url)}>
    <CopyIcon />
  </IconButton>
</div>
