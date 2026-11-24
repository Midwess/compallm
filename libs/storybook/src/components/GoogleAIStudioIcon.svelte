<script lang="ts">
  import type { SVGAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let { class: className, ...props }: SVGAttributes<SVGSVGElement> = $props()
  import googleAIStudioLogo from '../assets/provider-logos/GoogleAIStudioIcon.svg'
</script>

<svg
  {...props}
  viewBox="0 0 24 24"
  fill="none"
  aria-hidden={props['aria-label'] ? undefined : true}
  class={cx('size-4 shrink-0', className)}
>
  <image href={googleAIStudioLogo} width="24" height="24" />
</svg>
