<script lang="ts">
  import type { SVGAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let { class: className, ...props }: SVGAttributes<SVGSVGElement> = $props()
  const id = $props.id()
</script>

<svg
  {...props}
  viewBox="0 0 24 24"
  fill="none"
  aria-hidden={props['aria-label'] ? undefined : true}
  class={cx('size-4 shrink-0', className)}
>
  <g transform="translate(0.0 0.0) scale(0.7272727272727273)" fill="none">
    <g clip-path={`url(#${id}-groq-a)`}>
      <path fill="#F43E01" d="M.54.39h32v32h-32z" />
      <path fill="#fff" d="m18.445 4.406-9.468 13.74 7.341.665-1.69 9.578 9.469-13.74-7.342-.664 1.69-9.579Z" />
    </g>
    <defs>
      <clipPath id={`${id}-groq-a`}>
        <path fill="#fff" d="M.54.39h32v32h-32z" />
      </clipPath>
    </defs>
  </g>
</svg>
