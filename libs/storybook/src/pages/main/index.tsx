import type { ReactNode } from 'react'

export interface MainPageProps {
  sidebar: ReactNode
  children?: ReactNode
  contentLabel?: string
}

export function MainPage({ sidebar, children, contentLabel = 'Workspace content' }: MainPageProps) {
  return (
    <div data-ui="main-page" className="flex h-dvh min-h-0 w-full overflow-hidden bg-background font-sans text-foreground">
      {sidebar}
      <div className="min-h-0 min-w-0 flex-1 pt-4 pr-4 pb-2">
        <main
          aria-label={contentLabel}
          className="h-full min-h-0 min-w-0 overflow-auto overscroll-none rounded-xl
border-(length:--border-hairline) border-divider bg-surface"
        >
          {children}
        </main>
      </div>
    </div>
  )
}
