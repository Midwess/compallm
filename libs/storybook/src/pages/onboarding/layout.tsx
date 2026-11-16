import type { ReactNode } from 'react'
import { Button } from '../../components/buttons'
import { useOnboarding, useOnboardingController } from './controller'
import { OnboardingPanel } from './panel'

export function OnboardingLayout({
  title,
  description,
  progress,
  children,
  action,
}: {
  title: string
  description: string
  progress: number
  children: ReactNode
  action: ReactNode
}) {
  const controller = useOnboardingController()
  const step = useOnboarding((s) => s.step)
  const embedded = useOnboarding((s) => s.embedded)
  const artworkColor = useOnboarding((s) => s.artworkColor)
  const pending = useOnboarding((s) => s.pending)
  const canBack = ['profile', 'workspace', 'work', 'preferences', 'introduction'].includes(step)
  return (
    <div className={embedded ? 'grid place-items-center' : 'grid min-h-dvh place-items-center bg-background p-4 sm:p-8'}>
      <OnboardingPanel
        title={title}
        description={description}
        progress={progress}
        artworkColor={artworkColor}
        action={action}
        footer={
          <div
            aria-hidden={!canBack}
            inert={!canBack}
            className={[
              'mt-2 flex h-8 items-center',
              step === 'work' ? 'justify-between' : 'justify-center',
              canBack ? '' : 'invisible',
            ].join(' ')}
          >
            <Button variant="ghost" className="text-disabled" onClick={controller.back} loading={pending}>
              Back
            </Button>
            {step === 'work' ? (
              <Button variant="ghost" className="text-disabled" onClick={controller.skip} loading={pending}>
                I don’t want to share
              </Button>
            ) : null}
          </div>
        }
      >
        {children}
      </OnboardingPanel>
    </div>
  )
}
