<script lang="ts">
  import type { MainPageProps } from './index'
  import ContentValue from '../../components/Content.svelte'
  import ListIcon from 'phosphor-svelte/lib/ListIcon'

  let { sidebar, children, contentLabel = 'Workspace content' }: MainPageProps = $props()
  let open = $state(false)
  let toggle: HTMLButtonElement
  const navigationId = $props.id()
  function close() {
    open = false
    toggle?.focus({ preventScroll: true })
  }
</script>

<svelte:window
  onkeydown={(event) => {
    if (open && event.key === 'Escape') close()
  }}
/>
<div data-ui="main-page" class="relative flex h-dvh min-h-0 w-full overflow-hidden bg-background font-sans text-foreground">
  {#if open}<button
      type="button"
      aria-label="Close workspace navigation"
      class="fixed inset-0 z-30 bg-background/60 md:hidden"
      onclick={close}
    ></button>{/if}
  <div
    id={navigationId}
    data-workspace-navigation
    class={`fixed inset-y-0 left-0 z-40 h-full shrink-0 bg-sidebar md:static md:block ${open ? 'block' : 'hidden'}`}
    role="presentation"
    onclick={(event) => {
      if (event.target instanceof Element && event.target.closest('a[href]')) open = false
    }}
  >
    <ContentValue value={sidebar} />
  </div>
  <div class="flex min-h-0 min-w-0 flex-1 flex-col pt-2 pr-2 pb-2 pl-2 md:pt-4 md:pr-4 md:pl-0">
    <div class="flex h-10 shrink-0 items-center md:hidden">
      <button
        bind:this={toggle}
        type="button"
        aria-label="Open workspace navigation"
        aria-expanded={open}
        aria-controls={navigationId}
        class="rounded p-1 text-muted-foreground hover:text-foreground focus-visible:outline-ring"
        onclick={() => (open = !open)}
      >
        <ListIcon class="size-4" />
      </button>
    </div>
    <main
      aria-label={contentLabel}
      class="h-full min-h-0 min-w-0 overflow-x-auto overflow-y-scroll scrollbar-gutter-stable overscroll-y-none rounded-xl
border-(length:--border-hairline) border-divider bg-surface"
    >
      <ContentValue value={children} />
    </main>
  </div>
</div>
