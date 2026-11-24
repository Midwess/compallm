<script lang="ts">
  import type { Snippet } from 'svelte'
  import type { HTMLButtonAttributes } from 'svelte/elements'
  import CircleNotchIcon from 'phosphor-svelte/lib/CircleNotchIcon'
  import { cx } from './shared'
  import { interaction, loadingMotion } from './motion'
  export interface ButtonProps extends HTMLButtonAttributes {
    variant?: 'primary' | 'neutral' | 'ghost' | 'destructive' | 'link'
    size?: 'compact' | 'standard' | 'authentication' | 'content'
    loading?: boolean
    child?: Snippet<[HTMLButtonAttributes]>
    element?: HTMLButtonElement
  }
  let {
    variant = 'neutral',
    size = 'compact',
    loading = false,
    disabled = false,
    type = 'button',
    children,
    child,
    class: className,
    element = $bindable(),
    ...props
  }: ButtonProps = $props()
  const variants = {
    primary: 'bg-primary text-primary-foreground shadow-action enabled:hover:bg-primary-hover',
    neutral: 'bg-elevated text-foreground shadow-control enabled:hover:bg-elevated-hover',
    ghost: 'bg-transparent text-foreground shadow-none enabled:hover:bg-hover',
    destructive: 'bg-destructive text-destructive-foreground shadow-action enabled:hover:bg-destructive-hover',
    link: 'bg-transparent p-0 text-ring shadow-none enabled:hover:text-foreground !px-0',
  }
  const sizes = { compact: 'h-8', standard: 'h-10', authentication: 'h-12 w-88 text-ui-15', content: 'h-7' }
  const base =
    'inline-flex max-w-full shrink-0 cursor-pointer items-center justify-center gap-1 rounded-full border-(length:--border-hairline) border-transparent px-1.5 py-0 text-xsm leading-5 font-medium no-underline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring disabled:cursor-not-allowed disabled:opacity-50 disabled:shadow-none disabled:border-transparent [&_svg]:shrink-0'
  let attributes = $derived({
    ...props,
    type,
    disabled: disabled || loading,
    'aria-busy': loading || undefined,
    'aria-disabled': disabled || loading || props['aria-disabled'],
    onclick:
      disabled || loading
        ? (event: MouseEvent) => {
            event.preventDefault()
            event.stopPropagation()
          }
        : props.onclick,
    'data-disabled': disabled || loading ? '' : undefined,
    class: cx(base, sizes[size], variants[variant], size === 'compact' && variant === 'neutral' && 'h-6.5', className),
  })
</script>

{#if child}{@render child(attributes)}{:else}
  <button {...attributes} bind:this={element} use:interaction>
    {#if size === 'authentication'}
      <span class="relative inline-flex items-center justify-center gap-1">
        {@render children?.()}<span
          aria-hidden="true"
          class="absolute left-full ml-2 inline-flex size-3.5 items-center justify-center"
        >
          {#if loading}<span class="inline-flex" use:loadingMotion><CircleNotchIcon size={14} /></span>{/if}
        </span>
      </span>
    {:else}{#if loading}<span class="inline-flex" use:loadingMotion>
          <CircleNotchIcon size={14} aria-hidden="true" />
        </span>{/if}{@render children?.()}{/if}
  </button>
{/if}
