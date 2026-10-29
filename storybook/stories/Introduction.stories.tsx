import type { Meta, StoryObj } from '@storybook/react-vite'
import { Panel } from '../../portal/src/components/ui/Panel'

function Introduction() {
  return (
    <main className="design-system ui-stack" style={{ maxWidth: 860, margin: 'auto', padding: 24 }}>
      <p className="ui-muted">Compallm / Design workspace</p>
      <h1>Design the experience, together.</h1>
      <p>Review wireframes, refine components, then assemble screens. Everything here runs on sample data.</p>
      <Panel className="ui-stack">
        <h2>One prompt workflow</h2>
        <p>Keep Codex CLI open at the repository root. Ask it to inspect Storybook components and update a story. The browser preview refreshes as source files change.</p>
        <p className="ui-muted">MCP endpoint: http://localhost:6006/mcp · Check the connection with /mcp in Codex.</p>
      </Panel>
      <Panel className="ui-stack">
        <h2>Start with structure</h2>
        <p>Open Wireframes / Workspace to review a sample layout. Then use Components to review the shared controls. Foundations shows provisional dark surfaces and selected Pixelarticons.</p>
        <p className="ui-muted">These examples do not establish final navigation, brand colors, or implemented product features.</p>
      </Panel>
      <Panel className="ui-stack">
        <h2>Use the same components in the portal</h2>
        <p>Stories import React components directly from portal/src/components/ui. Review meaningful states here before connecting approved screens to routes and APIs.</p>
      </Panel>
    </main>
  )
}

const meta = { title: 'Start here/Design workspace', component: Introduction, parameters: { layout: 'fullscreen' } } satisfies Meta<typeof Introduction>
export default meta
type Story = StoryObj<typeof meta>
export const Overview: Story = {}
