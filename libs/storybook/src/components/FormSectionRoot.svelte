<script lang="ts">
  import { setSectionContext } from '../context'
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
  const id = $props.id()
  setSectionContext(id)
</script>

<section {...props} aria-labelledby={id} class={cx('group/section relative grid min-w-0 gap-4 pl-4', className)}>
  <div
    aria-hidden="true"
    class="absolute top-3 -bottom-9 -left-0.5 w-px -translate-x-1/2 bg-border group-last-of-type/section:hidden"
  ></div>
  {@render children?.()}
</section>
