import type { ReactNode } from 'react'

export interface EmptyStateProps {
  title: string
  description: string
  icon?: ReactNode
  action?: ReactNode
}

/** Contextual empty state; use a concise title and a useful next action. */
export function EmptyState({ title, description, icon, action }: EmptyStateProps) {
  return (
    <div className="ui-empty-state">
      {icon ? <div className="ui-empty-icon" aria-hidden="true">{icon}</div> : null}
      <h2>{title}</h2>
      <p className="ui-muted">{description}</p>
      {action}
    </div>
  )
}
