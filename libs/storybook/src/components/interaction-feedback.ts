export type InteractionFeedback = 'tick' | 'snap' | 'commit'

export function interactionFeedback(kind: InteractionFeedback): boolean {
  if (typeof navigator === 'undefined' || typeof navigator.vibrate !== 'function') return false
  try {
    return navigator.vibrate(kind === 'tick' ? 4 : kind === 'snap' ? 6 : 10)
  } catch {
    return false
  }
}
