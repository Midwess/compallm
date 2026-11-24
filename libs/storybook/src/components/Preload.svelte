<script lang="ts">
  import type { HTMLAnchorAttributes } from 'svelte/elements'
  import { interaction } from './motion'
  let {
    children,
    enabled = true,
    disabled = false,
    motion = false,
    href,
    onclick,
    ...props
  }: HTMLAnchorAttributes & { enabled?: boolean; disabled?: boolean; motion?: boolean } = $props()
  function animate(node: HTMLElement) {
    if (motion) return interaction(node)
  }
</script>

{#if href === undefined}
  <div class="contents" data-sveltekit-preload-data={enabled && !disabled ? 'hover' : false}>
    {@render children?.()}
  </div>
{:else}
  <a
    use:animate
    {...props}
    href={disabled ? undefined : href}
    aria-disabled={disabled || props['aria-disabled']}
    tabindex={disabled ? -1 : props.tabindex}
    data-sveltekit-preload-data={enabled && !disabled ? 'hover' : false}
    onclick={(event) => {
      if (disabled) {
        event.preventDefault()
        event.stopPropagation()
        return
      }
      onclick?.(event)
    }}
  >
    {@render children?.()}
  </a>
{/if}
