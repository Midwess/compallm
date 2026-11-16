import { useId, type ReactNode } from 'react'
import artworkImage from '../../assets/onboarding/midnight-penpot.webp'
import { Button } from '../../components/buttons'
import { useOnboarding, useOnboardingController } from './controller'

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
  const id = useId()
  const controller = useOnboardingController()
  const step = useOnboarding((state) => state.step)
  const embedded = useOnboarding((state) => state.embedded)
  const artworkColor = useOnboarding((state) => state.artworkColor)
  const pending = useOnboarding((state) => state.pending)
  return (
    <div className={embedded ? 'grid place-items-center' : 'grid min-h-dvh place-items-center bg-background p-4 sm:p-8'}>
      <main
        aria-labelledby={id}
        className="relative isolate flex min-h-112 w-full max-w-3xl flex-col overflow-hidden
rounded-3xl bg-card px-6 pt-6 pb-4 font-sans text-foreground ring-1 ring-border ring-inset md:min-h-120 md:px-6"
      >
        <div
          aria-hidden="true"
          className="pointer-events-none absolute inset-y-0 right-0 -z-10 isolate w-full
opacity-15 md:w-[45.833333%] md:opacity-25 dark:md:opacity-100"
          style={{
            maskImage: 'radial-gradient(ellipse 142.424% 111.905% at 157.576% 50%, black 85%, transparent 100%)',
          }}
        >
          <img
            src={artworkImage}
            alt=""
            className="absolute inset-0 size-full object-cover"
            style={{ filter: artworkColor ? 'grayscale(1)' : undefined }}
          />
          {artworkColor ? (
            <div className="absolute inset-0" style={{ backgroundColor: artworkColor, mixBlendMode: 'color' }} />
          ) : null}
        </div>
        <div
          role="progressbar"
          aria-label="Welcome progress"
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(progress * 100)}
          className="h-2 w-full max-w-180 shrink-0"
        >
          <div
            className="h-full rounded-full bg-primary"
            style={{
              width: `${progress * 100}%`,
              maskImage: 'linear-gradient(to right, black calc(100% - 48px), transparent)',
            }}
          />
        </div>
        <header className="relative mt-4 w-full max-w-110">
          <h1 id={id} className="flex min-h-11 items-center text-2xl leading-tight font-medium">
            {title}
          </h1>
          {description ? (
            <p
              className="flex min-h-8 max-w-100 items-center whitespace-pre-line text-sm
leading-6 text-muted-foreground"
            >
              {description}
            </p>
          ) : null}
        </header>
        <div className="relative mt-8 flex w-full max-w-96 flex-1 flex-col justify-start">{children}</div>
        <footer className="relative mt-auto w-full max-w-96 pt-4">
          {action}
          <div
            aria-hidden={!['profile', 'workspace', 'work', 'preferences', 'introduction'].includes(step)}
            inert={!['profile', 'workspace', 'work', 'preferences', 'introduction'].includes(step)}
            className={[
              'mt-2 flex h-8 items-center',
              step === 'work' ? 'justify-between' : 'justify-center',
              ['profile', 'workspace', 'work', 'preferences', 'introduction'].includes(step) ? '' : 'invisible',
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
        </footer>
      </main>
    </div>
  )
}
