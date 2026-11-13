import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { PageHeader, Title, Functions } from '../../src/components/page-header'
import { Button } from '../../src/components/buttons'
import { SearchField } from '../../src/components/fields'

function Demo({ long = false }: { long?: boolean }) {
  const [action, setAction] = useState('')
  return (
    <div data-ui="component-preview" className="min-h-64 bg-surface font-sans text-foreground">
      <PageHeader>
        <Title>{long ? 'A longer page title that truncates while keeping actions visible' : 'Members'}</Title>
        <Functions>
          <Button variant="ghost" onClick={() => setAction('Invite member')}>
            Invite member
          </Button>
        </Functions>
      </PageHeader>
      <div className="grid gap-6 px-4 py-6">
        <SearchField aria-label="Search members" placeholder="Search members…" className="max-w-70" />
        <p data-ui="content-start" className="text-xs text-muted-foreground">
          Content shares the header title’s left edge.
        </p>
        <output aria-label="Header action" className="text-xs">
          {action}
        </output>
      </div>
    </div>
  )
}

const meta = { title: 'Components/Navigation controls/Page header', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const Default: Story = { render: () => <Demo /> }
export const LongTitle: Story = { render: () => <Demo long /> }
