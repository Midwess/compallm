<script lang="ts">
  import { onDestroy } from 'svelte'
  import { setCollectionContext } from '../context'
  import { CollectionForms } from './collection-forms'
  import { scrollOwner } from './collection'
  import { cx, type ElementProps } from './shared'
  let {
    children,
    class: className,
    forms,
    scrollElement,
    ...props
  }: ElementProps & { forms?: CollectionForms; scrollElement?: HTMLElement | null } = $props()
  const owned = new CollectionForms()
  let owner = $state<HTMLElement | null>(null)
  setCollectionContext({
    get forms() {
      return forms ?? owned
    },
    get scrollElement() {
      return scrollElement ?? owner
    },
  })
  onDestroy(owned.dispose)
  function attach(node: HTMLElement) {
    owner = scrollOwner(node)
    const scroll = scrollElement ?? owner
    const target = scroll === document.scrollingElement ? window : scroll
    let frame = 0
    const update = () => {
      frame = 0
      const top = (scroll === document.scrollingElement ? 0 : scroll.getBoundingClientRect().top) + 32
      for (const body of node.querySelectorAll<HTMLElement>('[data-slot="table-category"]')) {
        const header = body.querySelector<HTMLElement>('[data-slot="table-category-header"]')
        header?.style.setProperty(
          '--category-offset',
          `${Math.min(0, body.getBoundingClientRect().bottom - top - header.offsetHeight)}px`,
        )
      }
    }
    const schedule = () => {
      if (!frame) frame = requestAnimationFrame(update)
    }
    target.addEventListener('scroll', schedule, { passive: true })
    const observer = new ResizeObserver(schedule)
    observer.observe(node)
    update()
    return {
      destroy() {
        target.removeEventListener('scroll', schedule)
        observer.disconnect()
        cancelAnimationFrame(frame)
      },
    }
  }
</script>

<div {...props} data-slot="table-container" class={cx('min-w-0 w-full max-w-full overflow-x-auto', className)} use:attach>
  {@render children?.()}
</div>
