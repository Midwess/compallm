import { twMerge } from 'tailwind-merge'

export function cx(...values: Array<string | undefined | false>) {
  return twMerge(values.filter(Boolean).join(' '))
}
export const popupMotion =
  'origin-top-left motion-safe:[&:popover-open]:animate-popup-in motion-safe:[&[open]]:animate-popup-in'
