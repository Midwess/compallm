import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { OnboardingPage, type OnboardingState } from '../../../src/pages/onboarding'
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
