import type {} from 'scheduler-polyfill'

let polyfill: Promise<unknown> | undefined

export async function yieldToBrowser(signal?: AbortSignal) {
  signal?.throwIfAborted()
  if (typeof window !== 'undefined') {
    if (!globalThis.scheduler?.yield) await (polyfill ??= import('scheduler-polyfill'))
    await globalThis.scheduler.yield()
  } else {
    await new Promise<void>((resolve) => setTimeout(resolve, 0))
  }
  signal?.throwIfAborted()
}

export async function processInSlices<T>(items: readonly T[], process: (item: T) => void, signal?: AbortSignal) {
  let deadline = performance.now() + 4
  for (const item of items) {
    signal?.throwIfAborted()
    process(item)
    if (performance.now() >= deadline) {
      await yieldToBrowser(signal)
      deadline = performance.now() + 4
    }
  }
}
