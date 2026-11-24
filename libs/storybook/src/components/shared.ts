import { extendTailwindMerge } from 'tailwind-merge'
import type { Snippet } from 'svelte'
import type { ClassValue } from 'clsx'
import { clsx } from 'clsx'
import type { HTMLAttributes } from 'svelte/elements'
const merge = extendTailwindMerge({ extend: { classGroups: { 'font-size': ['text-xsm'], 'font-weight': ['font-book'] } } })
export function cx(...values: unknown[]) {
  return merge(clsx(values as ClassValue[]))
}
export type Content = Snippet | string | number
export type ElementProps = HTMLAttributes<HTMLElement> & { children?: Snippet; element?: HTMLElement }
export function call(event: Event, handler?: (event: Event) => void) {
  handler?.(event)
  return !event.defaultPrevented
}
