import { untrack } from 'svelte'
import { animation } from './motion'

export function loadingPresentation(active: () => boolean, initial: () => boolean = active) {
  let visible = $state(false)
  let heldInitial = $state(false)
  let shownAt = 0
  $effect(() => {
    const pending = active()
    const first = initial()
    return untrack(() => {
      if (pending) {
        heldInitial = first
        if (visible) return
        const timer = setTimeout(() => {
          shownAt = Date.now()
          visible = true
        }, animation.skeleton.delayMs)
        return () => clearTimeout(timer)
      }
      if (visible) {
        const timer = setTimeout(
          () => {
            visible = false
            heldInitial = false
          },
          Math.max(0, animation.skeleton.minimumVisibleMs - (Date.now() - shownAt)),
        )
        return () => clearTimeout(timer)
      }
      heldInitial = false
    })
  })
  return {
    get visible() {
      return visible
    },
    get initial() {
      return active() ? initial() : heldInitial && visible
    },
  }
}
