import { useState, type ReactNode } from 'react'
import { OrganizationControllerContext } from '../../../src/context'
import { OrganizationPage, createOrganizationStore, type OrganizationController } from '../../../src/pages/organization'

class DemoOrganizationController implements OrganizationController {
  readonly store = createOrganizationStore({
    organizationId: 'compallm',
    organizations: [
      { id: 'compallm', name: 'Compallm' },
      { id: 'research', name: 'Research team' },
    ],
    providers: [
      { id: 'openai', name: 'OpenAI', href: '#openai' },
      { id: 'anthropic', name: 'Anthropic', href: '#anthropic' },
      { id: 'gemini', name: 'Gemini', href: '#gemini' },
    ],
    models: [
      { id: 'fast-chat', name: 'Fast Chat', href: '#fast-chat' },
      { id: 'sonnet', name: 'claude-3.5-sonnet', href: '#sonnet' },
    ],
    conversations: [
      { id: 'exploring', name: 'Exploring a new idea', href: '/chat' },
      { id: 'research', name: 'Research notes', href: '/chat' },
      { id: 'comparison', name: 'Model comparison', href: '/chat' },
    ],
  })
  constructor(private readonly report: (message: string) => void) {}
  getCurrentOrganization = () => {
    const state = this.store.getState()
    return state.organizations.find((organization) => organization.id === state.organizationId) ?? null
  }
  onOpenChat = (id?: string) => this.report(`Open chat: ${id ?? 'new'} → /chat`)
  onSelectOrganization = (id: string) => {
    this.report(`Switch organization: ${id}`)
    this.store.setState({ organizationId: id })
  }
  onSignOut = () => this.report('Sign out')
  onOpenSettings = () => this.report('Open settings')
  onSearch = () => this.report('Search workspace')
  onOpenDestination = (id: string) => {
    this.report(`Open destination: ${id}`)
    this.store.setState({ selectedDestination: id })
  }
  onCreateProvider = () => this.report('Create provider')
  onCreateRouter = () => this.report('Create router')
  onCreateModel = () => this.report('Create model')
}

export function OrganizationDemo({ empty = false, children }: { empty?: boolean; children?: ReactNode }) {
  const [intent, setIntent] = useState('')
  const [controller] = useState(() => {
    const result = new DemoOrganizationController(setIntent)
    if (empty) result.store.setState({ providers: [], models: [], conversations: [] })
    return result
  })
  return (
    <OrganizationControllerContext value={controller}>
      <OrganizationPage>{children}</OrganizationPage>
      <output aria-label="Controller intent" className="fixed right-8 bottom-12 text-xs text-muted">
        {intent}
      </output>
    </OrganizationControllerContext>
  )
}
