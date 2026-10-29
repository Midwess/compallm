import type { Meta, StoryObj } from '@storybook/react-vite'
import bookIcon from 'pixelarticons/svg/book-open.svg?raw'
import chatIcon from 'pixelarticons/svg/chat.svg?raw'
import checkIcon from 'pixelarticons/svg/check.svg?raw'
import { Panel } from '../../portal/src/components/ui/Panel'

function Foundations() {
  return (
    <main className="design-system ui-stack" style={{ maxWidth: 900, padding: 24 }}>
      <p className="ui-muted">For review / not a final brand system</p>
      <h1>A comfortable place to think.</h1>
      <p>Readable typography, warm dark surfaces, and small moments of nostalgia.</p>
      <div className="ui-row">
        {['--ui-bg', '--ui-surface', '--ui-elevated', '--ui-accent'].map((token) => (
          <div key={token} style={{ flex: '1 1 150px' }}>
            <div style={{ height: 88, background: `var(${token})`, border: '1px solid var(--ui-border)', borderRadius: 8 }} />
            <code>{token}</code>
          </div>
        ))}
      </div>
      <Panel className="ui-stack">
        <h2>Pixelarticons, used selectively</h2>
        <div className="ui-row">
          {[['Notebook', bookIcon], ['Conversation', chatIcon], ['Complete', checkIcon]].map(([label, svg]) => (
            <div key={label} className="ui-row">
              <span aria-hidden="true" style={{ display: 'inline-flex', width: 24, height: 24 }} dangerouslySetInnerHTML={{ __html: svg }} />
              <span>{label}</span>
            </div>
          ))}
        </div>
        <p className="ui-muted">Static SVG assets from the MIT-licensed Pixelarticons package. These are accent examples, not the final navigation icon set.</p>
      </Panel>
      <Panel className="ui-stack">
        <h2>Motion stays quiet</h2>
        <p>Use brief feedback for meaningful actions. Decorative elements stay still during reading. Honor reduced-motion preferences.</p>
      </Panel>
    </main>
  )
}

const meta = { title: 'Foundations/Direction for review', component: Foundations } satisfies Meta<typeof Foundations>
export default meta
type Story = StoryObj<typeof meta>
export const Dark: Story = {}
