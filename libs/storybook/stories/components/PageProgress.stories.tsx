import { useEffect, useRef, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { PageProgress } from '../../src/components/page-progress'
import { Button } from '../../src/components/buttons'
import { Preview } from './Preview'

function RequestDemo() {
  const [pending, setPending] = useState(0)
  const timers = useRef<ReturnType<typeof setTimeout>[]>([])
  useEffect(() => () => timers.current.forEach(clearTimeout), [])
  function request(duration: number) {
    setPending((count) => count + 1)
    timers.current.push(setTimeout(() => setPending((count) => count - 1), duration))
  }
  return (
    <Preview>
      <PageProgress active={pending > 0} />
      <section className="grid gap-3">
        <p className="text-muted">Pending requests: {pending}</p>
        <Button onClick={() => request(2500)}>Start request</Button>
        <Button
          onClick={() => {
            request(1500)
            request(4000)
          }}
        >
          Start overlapping requests
        </Button>
        <Button onClick={() => request(50)}>Fast request</Button>
      </section>
    </Preview>
  )
}

const meta = {
  title: 'Components/Feedback & overlays/Page progress',
  component: PageProgress,
  parameters: { layout: 'fullscreen' },
  args: { active: false },
} satisfies Meta<typeof PageProgress>
export default meta
type Story = StoryObj<typeof meta>

export const Requests: Story = { render: () => <RequestDemo /> }
export const Loading: Story = { args: { active: true } }
export const Idle: Story = {}
