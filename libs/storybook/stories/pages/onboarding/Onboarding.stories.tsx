import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { OnboardingDialog, OnboardingPage, type OnboardingState } from '../../../src/pages/onboarding'
import { Button } from '../../../src/components/buttons'
import { OnboardingControllerContext } from '../../../src/context'
import { DemoOnboardingController } from './demo-controller'

function Demo({ initial = {}, reject = false }: { initial?: Partial<OnboardingState>; reject?: boolean }) {
  const [controller] = useState(() => new DemoOnboardingController(initial, reject))
  useEffect(() => () => controller.stop(), [controller])
  return (
    <OnboardingControllerContext value={controller}>
      <OnboardingPage />
    </OnboardingControllerContext>
  )
}
const organization = {
  name: 'Acme',
  welcome: 'Welcome to the team. This is where we share ideas,\nexplore models, and build together.',
  links: [
    { label: 'Team handbook', href: 'https://example.com/handbook' },
    { label: 'Getting started', href: 'https://example.com/getting-started' },
  ],
  destination: 'Chat',
}
const meta = { title: 'Pages/Onboarding', component: OnboardingPage, parameters: { layout: 'fullscreen' } } satisfies Meta<
  typeof OnboardingPage
>
export default meta
type Story = StoryObj<typeof meta>
export const Welcome: Story = { render: () => <Demo /> }
export const Profile: Story = { render: () => <Demo initial={{ step: 'profile' }} /> }
export const WorkType: Story = { render: () => <Demo initial={{ step: 'work' }} /> }
export const Preferences: Story = { render: () => <Demo initial={{ step: 'preferences' }} /> }
export const Explore: Story = { render: () => <Demo initial={{ step: 'explore' }} /> }
export const Ready: Story = { render: () => <Demo initial={{ step: 'ready' }} /> }
export const InvitedNewUser: Story = { render: () => <Demo initial={{ invited: true, organization }} /> }
export const OrganizationWelcome: Story = {
  render: () => <Demo initial={{ step: 'organization', invited: true, existingUser: true, organization }} />,
}
export const OrganizationIntroduction: Story = {
  render: () => <Demo initial={{ step: 'introduction', invited: true, existingUser: true, organization }} />,
}
export const OrganizationReady: Story = {
  render: () => <Demo initial={{ step: 'ready', invited: true, existingUser: true, organization }} />,
}
export const SaveFailure: Story = { render: () => <Demo initial={{ step: 'profile' }} reject /> }
export const Light: Story = { globals: { theme: 'BASIC_WHITE' }, render: () => <Demo /> }
export const LongOrganization: Story = {
  render: () => (
    <Demo
      initial={{
        step: 'organization',
        invited: true,
        organization: {
          ...organization,
          name: 'A collaborative workspace for our international research team',
        },
      }}
    />
  ),
}

export const OtherWork: Story = { render: () => <Demo initial={{ step: 'work', workType: 'Architect' }} /> }

export const WorkspaceName: Story = { render: () => <Demo initial={{ step: 'workspace' }} /> }

export const Celebration: Story = { name: 'Workspace opening celebration', render: () => <Demo initial={{ step: 'ready' }} /> }

export const RequiredInvitedProfile: Story = {
  render: () => <Demo initial={{ step: 'profile', invited: true, requireFullName: true, requireAvatar: true, organization }} />,
}
export const CustomArtwork: Story = {
  render: () => <Demo initial={{ step: 'introduction', invited: true, artworkColor: '#9b5de5', organization }} />,
}

function PopupDemo({ firstPresentation = false, error = null }: { firstPresentation?: boolean; error?: string | null }) {
  const [open, setOpen] = useState(true)
  const [controller] = useState(() => new DemoOnboardingController({ embedded: true, step: 'profile', error }))
  return (
    <OnboardingControllerContext value={controller}>
      <Button onClick={() => setOpen(true)}>Open onboarding</Button>
      <OnboardingDialog open={open} onOpenChange={setOpen} firstPresentation={firstPresentation} />
    </OnboardingControllerContext>
  )
}
export const Popup: Story = { render: () => <PopupDemo /> }

export const FirstSignup: Story = { render: () => <PopupDemo firstPresentation /> }

export const PopupSaveFailure: Story = { render: () => <PopupDemo firstPresentation error="Username is already in use" /> }

export const SavingProfile: Story = { render: () => <Demo initial={{ step: 'profile', pending: true }} /> }
