<script lang="ts">
  import DOMPurify from 'dompurify'
  import { onMount } from 'svelte'
  import { cx } from './shared'
  let { message, navigate, class: className }: { message: string; navigate?: (href: string) => void; class?: string } = $props()
  let mounted = $state(false)
  onMount(() => (mounted = true))
  let html = $derived(
    mounted
      ? DOMPurify.sanitize(message, {
          ALLOWED_TAGS: ['a', 'strong', 'em', 'b', 'i', 'br', 'code'],
          ALLOWED_ATTR: ['href'],
          ALLOW_DATA_ATTR: false,
          ALLOW_ARIA_ATTR: false,
          ALLOWED_URI_REGEXP: /^(?:https?:\/\/|\/(?!\/)|#)/i,
        })
      : '',
  )
</script>

<span
  role="presentation"
  class={cx(
    'wrap-anywhere [&_a]:text-ring [&_a]:[text-decoration-line:var(--content-link-decoration)] [&_a]:hover:underline [&_a]:underline-offset-2 [&_a]:focus-visible:outline-2 [&_a]:focus-visible:outline-ring',
    className,
  )}
  onclick={(e) => {
    if (!navigate || e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return
    const anchor = e.target instanceof Element ? e.target.closest('a') : null
    const href = anchor?.getAttribute('href')
    if (!href || !e.currentTarget.contains(anchor)) return
    const target = new URL(href, window.location.href)
    if (target.origin !== window.location.origin) return
    e.preventDefault()
    navigate(target.pathname + target.search + target.hash)
  }}
>
  {#if mounted}<!-- eslint-disable svelte/no-at-html-tags -- DOMPurify sanitizes the shared allowlist above. -->{@html html}{:else}{message}{/if}
</span>
