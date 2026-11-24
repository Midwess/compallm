export function attachFieldError(
  anchor: HTMLElement,
  id: string,
  getBadge: () => HTMLButtonElement | null,
  setVisible: (visible: boolean) => void,
) {
  const originalPadding = anchor.style.paddingRight
  const originalBorder = anchor.style.borderColor
  const originalDescription = anchor.getAttribute('aria-describedby')
  anchor.setAttribute('aria-invalid', 'true')
  anchor.setAttribute('aria-describedby', [...new Set([...(originalDescription?.split(' ') ?? []), id])].join(' '))
  anchor.style.borderColor = 'var(--destructive)'
  const textControl = anchor.matches('input:not([type=file]), textarea')
  const padding = parseFloat(getComputedStyle(anchor).paddingRight)
  if (textControl) anchor.style.paddingRight = `${Math.max(padding, 32)}px`
  const update = () => {
    const node = getBadge()
    if (!node) return
    const rect = anchor.getBoundingClientRect()
    const avatar = anchor.matches('[data-ui="avatar-input"]')
    node.style.top = `${avatar ? rect.top - 4 : rect.top + Math.min(6, Math.max(0, (rect.height - 20) / 2))}px`
    node.style.left = `${rect.right - (avatar ? 16 : 26)}px`
    node.style.visibility = rect.width && rect.height && rect.bottom > 0 && rect.top < window.innerHeight ? 'visible' : 'hidden'
  }
  update()
  const observer = new ResizeObserver(update)
  observer.observe(anchor)
  if (anchor.parentElement) observer.observe(anchor.parentElement)
  const intersection = new IntersectionObserver(([entry]) => {
    setVisible(entry.isIntersecting)
    const node = getBadge()
    if (node) node.style.visibility = entry.isIntersecting ? 'visible' : 'hidden'
  })
  intersection.observe(anchor)
  window.addEventListener('resize', update)
  window.addEventListener('scroll', update, true)
  return () => {
    observer.disconnect()
    intersection.disconnect()
    window.removeEventListener('resize', update)
    window.removeEventListener('scroll', update, true)
    anchor.style.paddingRight = originalPadding
    anchor.style.borderColor = originalBorder
    anchor.removeAttribute('aria-invalid')
    if (originalDescription === null) anchor.removeAttribute('aria-describedby')
    else anchor.setAttribute('aria-describedby', originalDescription)
  }
}
