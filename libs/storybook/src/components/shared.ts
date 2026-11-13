import type { Ref } from 'react'
import { extendTailwindMerge } from 'tailwind-merge'

const twMerge = extendTailwindMerge({ extend: { classGroups: { 'font-size': ['text-xsm'] } } })

export function cx(...values: Array<string | undefined | false>) {
  return twMerge(values.filter(Boolean).join(' '))
}

export function composeRefs<T>(...refs: (Ref<T> | undefined)[]) {
  return (element: T | null) => {
    const cleanups = refs.map((ref) => {
      if (typeof ref === 'function') return ref(element)
      if (ref) ref.current = element
    })
    return () =>
      refs.forEach((ref, index) => {
        if (typeof cleanups[index] === 'function') cleanups[index]()
        else if (typeof ref === 'function') ref(null)
        else if (ref) ref.current = null
      })
  }
}
