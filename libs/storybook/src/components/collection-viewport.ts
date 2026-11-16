export class CollectionViewport {
  element: HTMLElement | null = null
  offset = 0
  height = 0
  width = 0
  scrolling = false
  private sections = new Map<HTMLElement, number>()
  register(section: HTMLElement, count: number) {
    this.sections.set(section, count)
    this.metadata()
    return () => {
      this.sections.delete(section)
      this.metadata()
    }
  }
  metadata() {
    const sections = [...this.sections]
      .filter(([element]) => element.isConnected)
      .sort(([a], [b]) => (a.compareDocumentPosition(b) & Node.DOCUMENT_POSITION_FOLLOWING ? -1 : 1))
    let offset = 1
    for (const [section, count] of sections) {
      const category = section.querySelector('[data-slot="table-category-header"]')
      if (category) {
        offset++
        category.setAttribute('aria-rowindex', String(offset))
      }
      section.querySelectorAll<HTMLElement>('[data-collection-key]').forEach((row) => {
        row.setAttribute('aria-rowindex', String(offset + Number(row.dataset.index) + 1))
      })
      offset += count
    }
    for (const [section] of sections) section.closest('table')?.setAttribute('aria-rowcount', String(offset))
  }
  private listeners = new Set<() => void>()
  private detach?: () => void
  observe = (listener: () => void) => {
    this.listeners.add(listener)
    listener()
    return () => {
      this.listeners.delete(listener)
    }
  }
  connect(element: HTMLElement) {
    if (this.element === element) return
    this.detach?.()
    this.element = element
    const anchoring = element.style.overflowAnchor
    element.style.overflowAnchor = 'none'
    const target = element === document.scrollingElement ? window : element
    let timer: ReturnType<typeof setTimeout> | undefined
    const update = () => {
      this.offset = element === document.scrollingElement ? window.scrollY : element.scrollTop
      this.height = element === document.scrollingElement ? window.innerHeight : element.clientHeight
      this.width = element === document.scrollingElement ? window.innerWidth : element.clientWidth
      for (const listener of this.listeners) listener()
    }
    const scroll = () => {
      this.scrolling = true
      update()
      clearTimeout(timer)
      timer = setTimeout(() => {
        this.scrolling = false
        update()
      }, 100)
    }
    const observer = new ResizeObserver(update)
    observer.observe(element)
    target.addEventListener('scroll', scroll, { passive: true })
    window.addEventListener('resize', update)
    update()
    this.detach = () => {
      element.style.overflowAnchor = anchoring
      observer.disconnect()
      target.removeEventListener('scroll', scroll)
      window.removeEventListener('resize', update)
      clearTimeout(timer)
    }
  }
  dispose = () => {
    this.detach?.()
    this.element = null
  }
}
