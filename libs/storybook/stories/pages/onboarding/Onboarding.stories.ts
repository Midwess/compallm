import PopupPreview from './Onboarding.stories-PopupDemo.svelte'
const organization = {
  name: 'Acme',
  welcome: 'Welcome to the team. This is where we share ideas,\nexplore models, and build together.',
  links: [
    { label: 'Team handbook', href: 'https://example.com/handbook' },
    { label: 'Getting started', href: 'https://example.com/getting-started' },
  ],
  destination: 'Chat',
}
import type { Meta, StoryObj } from '@storybook/svelte-vite'
import Preview from './Onboarding.stories-Demo.svelte'
const meta = { title: 'Pages/Onboarding', parameters: { layout: 'fullscreen' }, component: Preview } satisfies Meta<
  typeof Preview
>
export default meta
type Story = StoryObj<typeof meta>
export const Welcome: Story = { args: {} }
export const Profile: Story = { args: { initial: { step: 'profile' } } }
export const WorkType: Story = { args: { initial: { step: 'work' } } }
export const Preferences: Story = { args: { initial: { step: 'preferences' } } }
export const Explore: Story = { args: { initial: { step: 'explore' } } }
export const Ready: Story = { args: { initial: { step: 'ready' } } }
export const InvitedNewUser: Story = { args: { initial: { invited: true, organization } } }
export const OrganizationWelcome: Story = {
  args: { initial: { step: 'organization', invited: true, existingUser: true, organization } },
}
export const OrganizationIntroduction: Story = {
  args: { initial: { step: 'introduction', invited: true, existingUser: true, organization } },
}
export const OrganizationReady: Story = {
  args: { initial: { step: 'ready', invited: true, existingUser: true, organization } },
}
export const SaveFailure: Story = { args: { initial: { step: 'profile' }, reject: true } }
export const Light: Story = { globals: { theme: 'basic-white' }, args: {} }
export const LongOrganization: Story = {
  args: {
    initial: {
      step: 'organization',
      invited: true,
      organization: {
        ...organization,
        name: 'A collaborative workspace for our international research team',
      },
    },
  },
}
export const OtherWork: Story = { args: { initial: { step: 'work', workType: 'Architect' } } }
export const WorkspaceName: Story = { args: { initial: { step: 'workspace' } } }
export const Celebration: Story = { name: 'Workspace opening celebration', args: { initial: { step: 'ready' } } }
export const RequiredInvitedProfile: Story = {
  args: { initial: { step: 'profile', invited: true, requireFullName: true, requireAvatar: true, organization } },
}
export const CustomArtwork: Story = {
  args: { initial: { step: 'introduction', invited: true, artworkColor: '#9b5de5', organization } },
}
export const Popup: StoryObj<typeof PopupPreview> = { render: () => ({ Component: PopupPreview, props: {} }) }
export const FirstSignup: StoryObj<typeof PopupPreview> = {
  render: () => ({ Component: PopupPreview, props: { firstPresentation: true } }),
}
export const PopupSaveFailure: StoryObj<typeof PopupPreview> = {
  render: () => ({ Component: PopupPreview, props: { firstPresentation: true, error: 'Username is already in use' } }),
}
export const SavingProfile: Story = { args: { initial: { step: 'profile', pending: true } } }
