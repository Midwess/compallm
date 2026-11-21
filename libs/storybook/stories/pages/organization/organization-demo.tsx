import { DemoUserPreferenceController } from '../settings/preferences/user/demo-controller'
import { useState, type ReactNode } from 'react'
import { OrganizationControllerContext, UserPreferenceControllerContext } from '../../../src/context'
import {
  OrganizationPage,
  createOrganizationStore,
  type OrganizationController,
  type OrganizationState,
} from '../../../src/pages/organization'

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

export function OrganizationDemo({
  empty = false,
  sidebarLoading = false,
  children,
  sidebarOrder,
  sidebarLayout,
}: {
  empty?: boolean
  sidebarLoading?: boolean
  children?: ReactNode
  sidebarOrder?: OrganizationState['sidebarOrder']
  sidebarLayout?: OrganizationState['sidebarLayout']
}) {
  const [intent, setIntent] = useState('')
  const [preferences] = useState(() => {
    const value = new DemoUserPreferenceController()
    if (sidebarLoading) value.store.setState({ status: 'loading', saved: null })
    return value
  })
  const [controller] = useState(() => {
    const result = new DemoOrganizationController(setIntent)
    if (sidebarOrder)
      result.store.setState({
        sidebarOrder,
        sidebarLayout: result.store.getState().sidebarLayout.map((section) => ({
          ...section,
          items: [...section.items].sort((a, b) => sidebarOrder.indexOf(a) - sidebarOrder.indexOf(b)),
        })),
      })
    if (sidebarLayout) result.store.setState({ sidebarLayout })
    if (empty) result.store.setState({ providers: [], models: [], conversations: [] })
    return result
  })
  return (
    <OrganizationControllerContext value={controller}>
      <UserPreferenceControllerContext value={preferences}>
        <OrganizationPage>{children}</OrganizationPage>
      </UserPreferenceControllerContext>
      <output aria-label="Controller intent" className="fixed right-8 bottom-12 text-xs text-muted-foreground">
        {intent}
      </output>
    </OrganizationControllerContext>
  )
}
