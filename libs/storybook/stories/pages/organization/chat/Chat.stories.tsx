import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { ChatControllerContext } from '../../../../src/context'
import { ChatPage, createChatStore, type ChatController } from '../../../../src/pages/organization/chat'
import { OrganizationDemo } from '../organization-demo'

class DemoChatController implements ChatController {
  readonly store = createChatStore({
    modelId: 'fast',
    models: [
      { value: 'fast', label: 'Fast Chat' },
      { value: 'sonnet', label: 'claude-3.5-sonnet' },
    ],
  })
  constructor(private readonly report: (value: string) => void) {}
  onCreateChat = () => this.report('Create chat requested')
  onAttach = () => this.report('Attach file')
}

function Demo({ creating = false, noModels = false }: { creating?: boolean; noModels?: boolean }) {
  const [action, setAction] = useState('')
  const [controller] = useState(() => {
    const result = new DemoChatController(setAction)
    if (creating) result.store.setState({ creating: true, draft: 'Help me explore an idea.' })
    if (noModels) result.store.setState({ models: [], modelId: '' })
    return result
  })
  return (
    <ChatControllerContext value={controller}>
      <OrganizationDemo>
        <ChatPage />
      </OrganizationDemo>
      <output aria-label="Chat intent" className="sr-only">
        {action}
      </output>
    </ChatControllerContext>
  )
}

const meta = { title: 'Pages/organization/chat', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const NewChat: Story = { render: () => <Demo /> }
export const Creating: Story = { render: () => <Demo creating /> }
export const NoModels: Story = { render: () => <Demo noModels /> }
