import type { ReactNode } from 'react'

export interface MainPageProps {
  sidebar: ReactNode
  children?: ReactNode
  contentLabel?: string
}

export function MainPage({ sidebar, children, contentLabel = 'Workspace content' }: MainPageProps) {
  return (
    <div
      data-ui="main-page"
      className={`relative flex h-dvh min-h-0 w-full overflow-hidden bg-background font-sans text-foreground
      max-sm:has-data-model-settings:[&>aside]:hidden`}
    >
      {sidebar}
      <div className="min-h-0 min-w-0 flex-1 pt-4 pr-4 pb-2">
        <main
          aria-label={contentLabel}
          className="h-full min-h-0 min-w-0 overflow-x-auto overflow-y-scroll overscroll-y-none rounded-xl
border-(length:--border-hairline) border-divider bg-surface"
        >
          {children}
        </main>
      </div>
    </div>
  )
}
