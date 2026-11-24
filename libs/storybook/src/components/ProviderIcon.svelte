<script lang="ts">
  import ImageSquareIcon from 'phosphor-svelte/lib/ImageSquareIcon'
  import { logos, providerBrand } from './provider-icons'
  import type { SVGAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let {
    provider,
    logoPath,
    class: className,
    ...props
  }: SVGAttributes<SVGSVGElement> & { provider?: string | null; logoPath?: string } = $props()
  let brand = $derived(providerBrand(provider))
  let Icon = $derived(brand ? logos[brand] : ImageSquareIcon)
</script>

{#if logoPath}<img
    src={logoPath}
    alt={props['aria-label'] ?? ''}
    class={cx('size-4 shrink-0 rounded-sm bg-primary-foreground object-contain', className)}
  />{:else}{#if brand}<Icon
      class={cx('size-4 shrink-0', className)}
      aria-label={props['aria-label']}
      aria-hidden={props['aria-label'] ? undefined : true}
    />{:else}<ImageSquareIcon
      class={cx('size-4 shrink-0', className)}
      aria-hidden={props['aria-label'] ? undefined : true}
    />{/if}{/if}
