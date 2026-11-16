import { useEffect, useRef } from 'react'
import { Dialog } from '../../components/dialog'
import { Button } from '../../components/buttons'
import { Checkbox } from '../../components/selection'
import { SectionSurface } from '../../components/surfaces'
import { CelebrationMotion } from '../../components/motion'
import { Toast } from '../../components/feedback'
import { useOnboarding, useOnboardingController } from './controller'
import { OnboardingLayout } from './layout'
import { PersonalSetup } from './personal-setup'
export type { OnboardingController, OnboardingState, OnboardingStep, OnboardingProfile } from './types'

function Detail({ label, value }: { label: string; value: string }) {
  return (
    <SectionSurface className="flex min-h-15 flex-col justify-center px-4 py-2.5">
      <p className="text-sm leading-5 text-muted-foreground">{label}</p>
      <p className="mt-0.5 wrap-break-word text-base leading-6 font-medium">{value}</p>
    </SectionSurface>
  )
}

export function OnboardingPage() {
  const controller = useOnboardingController()
  const step = useOnboarding((s) => s.step)
  const profile = useOnboarding((s) => s.profile)
  const org = useOnboarding((s) => s.organization)
  const invited = useOnboarding((s) => s.invited)
  const existingUser = useOnboarding((s) => s.existingUser)
  const appearance = useOnboarding((s) => s.appearance)
  const error = useOnboarding((s) => s.error)
  const completed = useOnboarding((s) => s.completed)
  const pending = useOnboarding((s) => s.pending)
  const ready = step === 'ready' || completed
  const organization = step === 'organization' || (ready && invited)
  const introduction = step === 'introduction'
  const explore = step === 'explore'
  const title = ready
    ? 'You’re all set!'
    : organization
      ? `Welcome to ${org.name}`
      : introduction
        ? 'Make yourself at home'
        : explore
          ? 'Make room for your ideas'
          : 'Welcome to Compallm'
  const description = ready
    ? `Welcome to ${org.name}`
    : organization
      ? `A new space, and a team to explore it with.\nMake yourself at home in ${org.name}.`
      : introduction
        ? org.welcome || 'Your team’s workspace is ready.\nHere are a few useful places to start.'
        : explore
          ? 'Find a little inspiration for what comes next.'
          : 'A workspace for your ideas, conversations, and AI models.\nLet’s make it yours.'
  return (
    <div
      className="relative"
      data-theme={appearance === 'SYSTEM' ? undefined : appearance === 'LIGHT' ? 'basic-white' : 'basic-dark'}
    >
      {['welcome', 'profile', 'workspace', 'work', 'preferences'].includes(step) && !completed ? (
        <PersonalSetup key={step} step={step} />
      ) : (
        <OnboardingLayout
          title={title}
          description={description}
          progress={ready ? 1 : organization ? (existingUser ? 0.65 : 0.8) : introduction ? 0.9 : explore ? 0.58 : 0.28}
          action={
            <Button
              loading={pending}
              variant="primary"
              size="authentication"
              className="h-12 w-full text-base"
              onClick={ready ? controller.finish : controller.next}
            >
              {ready ? (invited ? `Open ${org.name}` : 'Open my workspace') : 'Continue'}
            </Button>
          }
        >
          {ready ? (
            <div className="relative h-48 w-full" aria-label="Workspace opening celebration">
              <CelebrationMotion active={!completed} />
            </div>
          ) : explore ? (
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
              {['Chat', 'Models', 'Providers', 'Plugins'].map((name) => (
                <SectionSurface key={name} className="flex min-h-13 items-center gap-3 px-4 py-3">
                  <label className="flex w-full cursor-pointer items-center gap-3 text-base font-medium">
                    <Checkbox aria-label={name} defaultChecked />
                    {name}
                  </label>
                </SectionSurface>
              ))}
            </div>
          ) : introduction ? (
            <div className="grid gap-3">
              {org.links.map((link) => (
                <SectionSurface key={link.href} className="p-0">
                  <Button asChild variant="ghost" className="h-15 w-full justify-start px-4 text-base">
                    <a href={link.href} target="_blank" rel="noopener noreferrer">
                      {link.label} ↗
                    </a>
                  </Button>
                </SectionSurface>
              ))}
              {!org.links.length ? <Detail label="Your organization" value={org.name} /> : null}
            </div>
          ) : (
            <div className="grid gap-3">
              <Detail
                label={ready ? (invited ? 'Your organization' : org.name) : organization ? 'Signed in as' : 'Your account'}
                value={ready ? (invited ? org.name : 'Your personal organization') : profile.email}
              />
              <Detail
                label={ready ? 'Your role' : organization ? 'Organization' : 'Your workspace'}
                value={ready ? (invited ? 'Member' : 'Owner') : org.name}
              />
            </div>
          )}
        </OnboardingLayout>
      )}
      {error ? <OnboardingToast message={error} onDismiss={controller.dismissError} /> : null}
    </div>
  )
}

export { OnboardingPreviewController } from './preview-controller'

export function OnboardingDialog({
  open,
  onOpenChange,
  firstPresentation = false,
}: {
  open: boolean
  onOpenChange(open: boolean): void
  firstPresentation?: boolean
}) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content
        onCancel={(event) => event.preventDefault()}
        className={[
          'w-2xl max-w-[calc(100%-2rem)] rounded-3xl border-0 bg-transparent',
          firstPresentation ? 'backdrop:bg-background' : 'backdrop:bg-transparent',
        ].join(' ')}
      >
        <Dialog.Title className="sr-only">Welcome to your workspace</Dialog.Title>
        <OnboardingPage />
        <Button variant="ghost" className="absolute top-9 right-6" onClick={() => onOpenChange(false)}>
          Close
        </Button>
      </Dialog.Content>
    </Dialog.Root>
  )
}

function OnboardingToast({ message, onDismiss }: { message: string; onDismiss(): void }) {
  const host = useRef<HTMLDivElement>(null)
  useEffect(() => {
    const element = host.current
    const frame = requestAnimationFrame(() => element?.showPopover())
    return () => {
      cancelAnimationFrame(frame)
      element?.hidePopover()
    }
  }, [])
  return (
    <div
      ref={host}
      popover="manual"
      className="fixed inset-auto top-4 right-4 m-0 max-w-[calc(100%-2rem)] border-0 bg-transparent p-0 text-foreground"
    >
      <Toast tone="error" message={message} onDismiss={onDismiss} />
    </div>
  )
}
