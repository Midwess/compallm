import type { HTMLAttributes } from 'react'

/** Surface container shared by the portal and design previews. */
export function Panel({ className = '', ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={`ui-panel ${className}`} />
}
