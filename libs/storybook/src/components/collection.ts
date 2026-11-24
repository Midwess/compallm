import type { CollectionPaging } from './collection-types'
export function scrollOwner(node: HTMLElement): HTMLElement {
  let parent = node.parentElement
  while (parent) {
    const style = getComputedStyle(parent)
    if (/(auto|scroll)/.test(style.overflowY) && (style.overflowX !== 'auto' || parent.scrollHeight > parent.clientHeight + 1))
      return parent
    parent = parent.parentElement
  }
  return document.scrollingElement as HTMLElement
}
export function loadMore(node: HTMLElement, paging: (CollectionPaging & { scrollElement?: HTMLElement | null }) | undefined) {
  let observer: IntersectionObserver | undefined
  const update = (next: (CollectionPaging & { scrollElement?: HTMLElement | null }) | undefined) => {
    observer?.disconnect()
    if (!next?.hasMore || next.loading || next.error) return
    const owner = next.scrollElement ?? scrollOwner(node)
    observer = new IntersectionObserver(
      (entries) => {
        if (entries[0]?.isIntersecting) {
          observer?.disconnect()
          next.onLoadMore()
        }
      },
      { root: owner === document.scrollingElement ? null : owner, rootMargin: '160px' },
    )
    observer.observe(node)
  }
  update(paging)
  return {
    update,
    destroy() {
      observer?.disconnect()
    },
  }
}
