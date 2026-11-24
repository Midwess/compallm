<script lang="ts">
  import Preload from './Preload.svelte'

  import Content from './Content.svelte'
  import { cx, type Content as Value } from './shared'
  let {
    label,
    href,
    icon,
    selected = false,
    onNavigate,
    class: className,
    preload = false,
  }: {
    label: string
    href: string
    icon?: Value
    selected?: boolean
    onNavigate?: (href: string) => void
    preload?: boolean
    class?: string
  } = $props()
</script>

<Preload
  motion
  {href}
  enabled={preload}
  aria-current={selected ? 'page' : undefined}
  title={label}
  class={cx(
    'relative -mx-0.5 flex h-7 min-w-0 items-center gap-1.5 rounded-xl border-y border-transparent bg-clip-padding px-2.5 text-xsm font-medium leading-4.25 text-muted-foreground hover:bg-hover hover:text-foreground aria-[current=page]:bg-selection aria-[current=page]:text-selection-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring',
    className,
  )}
  onclick={(e) => {
    if (!onNavigate || e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return
    e.preventDefault()
    onNavigate(href)
  }}
>
  <span aria-hidden="true" class="flex size-3.5 shrink-0 items-center justify-center [&>svg]:size-3.5">
    <Content value={icon} />
  </span><span class="min-w-0 truncate">{label}</span>
</Preload>
