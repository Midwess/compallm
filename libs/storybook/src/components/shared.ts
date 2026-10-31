import { twMerge } from 'tailwind-merge'

export function cx(...values: Array<string | undefined | false>) {
  return twMerge(values.filter(Boolean).join(' '))
}
