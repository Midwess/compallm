import { useEffect, useRef, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { PageProgress } from '../../src/components/page-progress'
import { Button } from '../../src/components/buttons'
import { Preview } from './Preview'
import { Dialog } from '../../src/components/dialog'

function RequestDemo() {
  const [pending, setPending] = useState(0)
  const [result, setResult] = useState('')
  const timers = useRef<ReturnType<typeof setTimeout>[]>([])
  useEffect(() => () => timers.current.forEach(clearTimeout), [])
  function request(duration: number, failed = false) {
    setPending((count) => count + 1)
    setResult('')
    timers.current.push(
      setTimeout(() => {
        setPending((count) => count - 1)
        setResult(failed ? 'Request failed; loading finished.' : 'Request completed.')
      }, duration),
    )
  }
  return (
    <Preview>
      <PageProgress active={pending > 0} />
      <section className="grid gap-3">
        <p className="text-muted-foreground">Pending requests: {pending}</p>
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
        <Button onClick={() => request(50, true)}>Failed request</Button>
        {result ? (
          <p role="status" className="text-muted-foreground">
            {result}
          </p>
        ) : null}
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

function DialogProgress() {
  const [open, setOpen] = useState(false)
  return (
    <Preview>
      <PageProgress active />
      <Button onClick={() => setOpen(true)}>Customize sidebar</Button>
      {open ? (
        <Dialog.Root
          open
          onOpenChange={(open) => {
            if (!open) setOpen(false)
          }}
        >
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title>Customize sidebar</Dialog.Title>
              <Dialog.Close />
            </Dialog.Header>
            <Dialog.Body>The request progress bar remains above this dialog.</Dialog.Body>
          </Dialog.Content>
        </Dialog.Root>
      ) : null}
    </Preview>
  )
}
export const AboveDialog: Story = { render: () => <DialogProgress /> }
