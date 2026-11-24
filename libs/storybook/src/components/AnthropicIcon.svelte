<script lang="ts">
  import type { SVGAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let { class: className, ...props }: SVGAttributes<SVGSVGElement> = $props()
</script>

<svg
  {...props}
  viewBox="0 0 24 24"
  fill="none"
  aria-hidden={props['aria-label'] ? undefined : true}
  class={cx('size-4 shrink-0', className)}
>
  <g transform="translate(0.0 3.7714285714285722) scale(0.6857142857142857)" fill="none">
    <path
      d={'M-0.00143743 24L9.56656 -1.90735e-06H15.0546L24.6226 24H19.2626L17.3106 18.96H7.29456L5.34256 24' +
        'H-0.00143743ZM9.02256 14.496H15.5826L12.3026 6.064L9.02256 14.496ZM29.0386 24L19.4706 -1.90735e-' +
        '06H24.7186L34.2866 24H29.0386Z'}
      fill="currentColor"
    />
  </g>
</svg>
