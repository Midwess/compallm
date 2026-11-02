import type { ReactNode } from 'react'

export interface MainPageProps {
  sidebar: ReactNode
  children?: ReactNode
  contentLabel?: string
}

export function MainPage({ sidebar, children, contentLabel = 'Workspace content' }: MainPageProps) {
  return (
    <div data-ui="main-page" className="flex h-svh min-h-0 w-full overflow-hidden bg-background font-sans text-text">
      {sidebar}
      <div className="min-h-0 min-w-0 flex-1 pt-4 pr-4 pb-8">
        <main
          aria-label={contentLabel}
          className="h-full min-h-0 min-w-0 overflow-auto rounded-xl border-[0.5px] border-divider bg-surface"
        >
          {children}
        </main>
      </div>
    </div>
  )
}
