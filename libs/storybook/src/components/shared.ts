import { extendTailwindMerge } from 'tailwind-merge'

const twMerge = extendTailwindMerge({ extend: { classGroups: { 'font-size': ['text-xsm'] } } })

export function cx(...values: Array<string | undefined | false>) {
  return twMerge(values.filter(Boolean).join(' '))
}
