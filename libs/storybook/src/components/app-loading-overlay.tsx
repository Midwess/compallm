import { AnimatePresence, motion } from 'motion/react'
import { Button } from './buttons'
import { CubeGridMotion, useTransition } from './motion'

export interface AppLoadingOverlayProps {
  active: boolean
  error?: string
  onRetry?: () => void
}

export function AppLoadingOverlay({ active, error, onRetry }: AppLoadingOverlayProps) {
  const timing = useTransition('exit')
  return (
    <AnimatePresence>
      {active ? (
        <motion.div
          key="app-loading"
          className="fixed inset-0 z-100 flex flex-col items-center justify-center gap-6
bg-inset bg-radial-[at_50%_42%] from-elevated to-inset to-70% px-6 text-foreground"
          initial={false}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={timing}
          aria-label="Workspace loading"
        >
          <CubeGridMotion stopped={Boolean(error)} />
          {error ? (
            <p role="alert" className="max-w-sm text-center text-xsm text-muted-foreground">
              {error}
            </p>
          ) : (
            <span role="status" className="sr-only">
              Loading your workspace…
            </span>
          )}
          {error && onRetry ? <Button onClick={onRetry}>Retry</Button> : null}
        </motion.div>
      ) : null}
    </AnimatePresence>
  )
}
