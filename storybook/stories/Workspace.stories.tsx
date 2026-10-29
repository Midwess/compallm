import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Button } from '../../portal/src/components/ui/Button'
import { Panel } from '../../portal/src/components/ui/Panel'
import { TextField } from '../../portal/src/components/ui/TextField'
import { EmptyState } from '../../portal/src/components/ui/EmptyState'
import './wireframe.css'

function WorkspaceWireframe({ populated = false }: { populated?: boolean }) {
  const [active, setActive] = useState('Chat')
  return (
    <div className="design-system wireframe">
      <div className="wireframe-notice">WIREFRAME · Layout proposal · Sample data · No backend connection</div>
      <div className="wireframe-layout">
        <aside className="wireframe-sidebar">
          <strong>Compallm</strong>
          <TextField label="Workspace" value="Personal workspace" readOnly />
          <nav aria-label="Workspace preview" className="ui-stack">
            {['Chat', 'Notebooks', 'Usage', 'Settings'].map((item) => (
              <Button key={item} variant={active === item ? 'secondary' : 'ghost'} aria-current={active === item ? 'page' : undefined} onClick={() => setActive(item)}>{item}</Button>
            ))}
          </nav>
          <p className="ui-muted">Navigation labels and organization are open for review.</p>
        </aside>
        <main className="wireframe-main">
          <header className="wireframe-header"><h1>{active}</h1><span className="ui-muted">Screen composition preview</span></header>
          <div className="wireframe-content ui-stack">
            {active === 'Chat' ? (
              <>
                {populated ? (
                  <Panel className="ui-stack">
                    <strong>You</strong><p>Help me organize what I'm learning.</p>
                    <strong>Assistant · Example model</strong><p>We can start with a question, gather sources, and turn the useful parts into a notebook.</p>
                  </Panel>
                ) : <EmptyState title="What would you like to explore?" description="This space previews the reading area. Model controls and conversation navigation will be designed separately." />}
                <Panel className="ui-stack">
                  <TextField label="Message preview" placeholder="Ask a question…" hint="Preview only. Messages are not sent." />
                  <div><Button disabled>Send message</Button></div>
                </Panel>
              </>
            ) : <EmptyState title={`${active} layout to explore`} description="Use this space to discuss the next screen. Its behavior and final design are not implemented." />}
          </div>
        </main>
      </div>
    </div>
  )
}

const meta = {
  title: 'Wireframes/Workspace',
  component: WorkspaceWireframe,
  parameters: { layout: 'fullscreen' },
} satisfies Meta<typeof WorkspaceWireframe>
export default meta
type Story = StoryObj<typeof meta>
export const Empty: Story = {}
export const Conversation: Story = { args: { populated: true } }
export const Mobile: Story = { globals: { viewport: { value: 'mobile1', isRotated: false } } }
