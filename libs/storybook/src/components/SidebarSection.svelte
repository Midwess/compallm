<script lang="ts">
  import { untrack, type Snippet } from 'svelte'
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import { sidebarDisclosure } from './motion'
  import { cx, type Content as Value } from './shared'
  import Content from './Content.svelte'
  const id = $props.id()
  let {
    label,
    children,
    open = $bindable(true),
    defaultOpen = true,
    onOpenChange,
    onAdd,
    addLabel,
    icon,
    folder = false,
    class: className,
  }: {
    label: string
    children?: Snippet
    open?: boolean
    defaultOpen?: boolean
    onOpenChange?: (value: boolean) => void
    onAdd?: () => void
    addLabel?: string
    folder?: boolean
    icon?: Value
    class?: string
  } = $props()
  untrack(() => {
    if (!defaultOpen) open = false
  })
</script>

<div class={cx('relative shrink-0', className)}>
  <details
    use:sidebarDisclosure={{
      open,
      onOpenChange: (next) => {
        open = next
        onOpenChange?.(next)
      },
    }}
  >
    <summary
      aria-controls={id}
      class={cx(
        'flex min-h-7 cursor-pointer list-none items-center gap-1 rounded-xl px-2 pr-7 text-xs font-medium leading-3 text-muted-foreground marker:hidden [&::-webkit-details-marker]:hidden',
        folder &&
          'relative -mx-0.5 gap-1.5 border-y border-transparent bg-clip-padding px-2.5 pr-7.5 text-xsm font-normal leading-4 hover:bg-hover',
      )}
    >
      {#if folder}<span aria-hidden="true" class="flex size-3.5 shrink-0 items-center justify-center [&>svg]:size-3.5">
          <Content value={icon} />
        </span>{/if}<span class="min-w-0 truncate leading-normal">{label}</span><svg
        width="7"
        height="7"
        viewBox="0 0 8 8"
        aria-hidden="true"
        data-disclosure-caret
      >
        <path
          d="M2 1.72h4a.4.4 0 0 1 .35.6l-2 3.96a.4.4 0 0 1-.7 0l-2-3.96a.4.4 0 0 1 .35-.6Z"
          fill="currentColor"
          stroke="currentColor"
          stroke-width="0.8"
          stroke-linejoin="round"
        />
      </svg>
    </summary>
    <div {id} data-disclosure-body class={folder ? 'mt-0.5 grid pl-4 pr-4 [&_a]:font-normal' : 'grid'}>
      {@render children?.()}
    </div>
  </details>
  {#if onAdd}<button
      type="button"
      aria-label={addLabel ?? `Add ${label.toLowerCase()}`}
      class="absolute top-1.5 right-0 size-4 rounded-full text-foreground"
      onclick={() => {
        if (folder) {
          open = true
          onOpenChange?.(true)
        }
        onAdd?.()
      }}
    >
      <PlusIcon size={8.28} />
    </button>{/if}
</div>
