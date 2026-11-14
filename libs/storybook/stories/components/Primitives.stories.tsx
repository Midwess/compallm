import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { useState } from 'react'
import { Preview } from './Preview'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Button } from '../../src/components/buttons'
import { Dialog, AlertDialog } from '../../src/components/dialog'
import { Popup } from '../../src/components/popup'
import { Table } from '../../src/components/table'
import { FormField } from '../../src/components/form-field'
import { Searchbox } from '../../src/components/fields'
import { RichMessage } from '../../src/components/rich-message'
import { Checkbox, Switch } from '../../src/components/selection'

const meta = {
  title: 'Components/Composition primitives',
  parameters: { layout: 'padded' },
  decorators: [
    (Story) => (
      <Preview>
        <Story />
      </Preview>
    ),
  ],
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function DialogDemo() {
  const [open, setOpen] = useState(false)
  return (
    <>
      <Button onClick={() => setOpen(true)}>Open dialog</Button>
      {open ? (
        <Dialog.Root
          open
          onOpenChange={(open) => {
            if (!open) setOpen(false)
          }}
        >
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title>Edit details</Dialog.Title>
              <Dialog.Close />
            </Dialog.Header>
            <Dialog.Discard />
            <Dialog.Body>
              <FormField.Root>
                <FormField.Label>Name</FormField.Label>
                <FormField.Input autoFocus placeholder="Enter name" />
              </FormField.Root>
            </Dialog.Body>
            <Dialog.Footer>
              <Dialog.Cancel />
              <Button variant="primary" onClick={() => setOpen(false)}>
                Save
              </Button>
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog.Root>
      ) : null}
    </>
  )
}
function AlertDemo() {
  const [open, setOpen] = useState(false)
  return (
    <>
      <Button onClick={() => setOpen(true)}>Confirm discard</Button>
      {open ? (
        <AlertDialog.Root
          open
          onOpenChange={(open) => {
            if (!open) setOpen(false)
          }}
        >
          <AlertDialog.Content>
            <AlertDialog.Header>
              <AlertDialog.Title>Discard changes?</AlertDialog.Title>
            </AlertDialog.Header>
            <AlertDialog.Body>
              <AlertDialog.Description>Your unsaved changes will be lost.</AlertDialog.Description>
            </AlertDialog.Body>
            <AlertDialog.Footer>
              <Button autoFocus onClick={() => setOpen(false)}>
                Keep editing
              </Button>
              <Button variant="destructive" onClick={() => setOpen(false)}>
                Discard changes
              </Button>
            </AlertDialog.Footer>
          </AlertDialog.Content>
        </AlertDialog.Root>
      ) : null}
    </>
  )
}
export const DialogComposition: Story = { render: () => <DialogDemo /> }
export const AlertComposition: Story = { render: () => <AlertDemo /> }
export const PopupComposition: Story = {
  render: () => (
    <Popup.Root>
      <Popup.Trigger aria-label="Open actions">Actions</Popup.Trigger>
      <Popup.Content aria-label="Actions">
        <Popup.Item>Clone</Popup.Item>
        <Popup.Item>Settings</Popup.Item>
      </Popup.Content>
    </Popup.Root>
  ),
}
export const TableComposition: Story = {
  render: () => (
    <Table.Root>
      <Table.Content aria-label="Sample models">
        <Table.Header>
          <Table.Row>
            <Table.Head>Model</Table.Head>
            <Table.Head>Input price</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          <Table.Row>
            <Table.Cell>Sample model</Table.Cell>
            <Table.Cell>$2.00</Table.Cell>
          </Table.Row>
        </Table.Body>
      </Table.Content>
    </Table.Root>
  ),
}
export const CategorizedTable: Story = {
  render: () => (
    <div className="max-h-64 max-w-160 overflow-auto rounded-lg bg-inset p-4">
      <Table.Root className="overflow-visible">
        <Table.Content className="border-separate border-spacing-0" aria-label="Categorized table example">
          <Table.Header className="sticky top-0 z-20 bg-inset">
            <Table.Row>
              <Table.Head>Name</Table.Head>
              <Table.Head>Scope</Table.Head>
              <Table.Head>Allow</Table.Head>
            </Table.Row>
          </Table.Header>
          {['Models', 'Providers', 'Groups'].map((category) => (
            <Table.Category key={category} label={category} count="0 / 5" action={<Switch aria-label={'Allow ' + category} />}>
              {Array.from({ length: 5 }, (_, i) => (
                <Table.Row key={i}>
                  <Table.Cell>
                    {category} action {i + 1}
                  </Table.Cell>
                  <Table.Cell>Sample scope</Table.Cell>
                  <Table.Cell>
                    <Switch aria-label={category + ' action ' + (i + 1)} />
                  </Table.Cell>
                </Table.Row>
              ))}
            </Table.Category>
          ))}
        </Table.Content>
      </Table.Root>
    </div>
  ),
}
export const ControlsComposition: Story = {
  render: () => (
    <div className="grid max-w-md gap-4">
      <FormField.Root>
        <FormField.Label>Connection name</FormField.Label>
        <FormField.Input placeholder="Enter name" />
        <FormField.Hint>A name for this connection.</FormField.Hint>
      </FormField.Root>
      <Searchbox.Root>
        <Searchbox.Input aria-label="Search models" placeholder="Search models…" />
      </Searchbox.Root>
      <label className="flex items-center gap-2 text-sm">
        <Checkbox defaultChecked />
        Enabled
      </label>
    </div>
  ),
}

export const RichMessageContent: Story = {
  render: () => <RichMessage message={'Email already exists, <a href="/login">login here</a>'} />,
}

function FilterDemo({ edge = false }: { edge?: boolean }) {
  const [status, setStatus] = useState('all')
  const [modelType, setModelType] = useState('all')
  const [capabilities, setCapabilities] = useState<string[]>([])
  const [outsideClicks, setOutsideClicks] = useState(0)
  return (
    <div className={edge ? 'flex h-80 flex-col items-end justify-end gap-4 overflow-hidden' : 'grid gap-4'}>
      <Popup.Root>
        <Popup.IconTrigger aria-label="Filter example">
          <FunnelIcon aria-hidden="true" className="size-3.5" />
        </Popup.IconTrigger>
        <Popup.Content aria-label="Filter example">
          <Popup.Sub>
            <Popup.SubTrigger>Selection</Popup.SubTrigger>
            <Popup.SubContent aria-label="Selection">
              <Popup.RadioGroup value={status} onValueChange={setStatus}>
                {['all', 'enabled', 'disabled'].map((value) => (
                  <Popup.RadioItem key={value} value={value}>
                    {value === 'all' ? 'All policies' : value === 'enabled' ? 'Enabled' : 'Disabled'}
                  </Popup.RadioItem>
                ))}
              </Popup.RadioGroup>
            </Popup.SubContent>
          </Popup.Sub>
          <Popup.Sub>
            <Popup.SubTrigger>Model type</Popup.SubTrigger>
            <Popup.SubContent aria-label="Model type">
              <Popup.RadioGroup value={modelType} onValueChange={setModelType}>
                {['all', 'OpenAI Completion', 'OpenAI Response', 'Anthropic'].map((value) => (
                  <Popup.RadioItem key={value} value={value}>
                    {value === 'all' ? 'All model types' : value}
                  </Popup.RadioItem>
                ))}
              </Popup.RadioGroup>
            </Popup.SubContent>
          </Popup.Sub>
          <Popup.Sub>
            <Popup.SubTrigger>Capabilities</Popup.SubTrigger>
            <Popup.SubContent aria-label="Capabilities">
              {['Vision', 'Tools', 'Reasoning'].map((value) => (
                <Popup.CheckboxItem
                  key={value}
                  checked={capabilities.includes(value)}
                  onCheckedChange={(checked) =>
                    setCapabilities((current) => (checked ? [...current, value] : current.filter((item) => item !== value)))
                  }
                >
                  {value}
                </Popup.CheckboxItem>
              ))}
            </Popup.SubContent>
          </Popup.Sub>
          <Popup.Separator />
          <Popup.Item
            onSelect={() => {
              setStatus('all')
              setModelType('all')
              setCapabilities([])
            }}
          >
            Clear all filters
          </Popup.Item>
        </Popup.Content>
      </Popup.Root>
      <p className="text-xs text-muted-foreground" aria-live="polite">
        Selection: {status} · Model type: {modelType} · Capabilities: {capabilities.join(', ') || 'all'}
      </p>
      <Button onClick={() => setOutsideClicks((count) => count + 1)}>Outside action · {outsideClicks}</Button>
    </div>
  )
}

export const FilterSubmenus: Story = { render: () => <FilterDemo /> }
export const FilterAtContainerEdge: Story = { render: () => <FilterDemo edge /> }

function ChildCompositionDemo() {
  const [clicks, setClicks] = useState(0)
  return (
    <div className="grid justify-items-start gap-4">
      <Dialog.Root>
        <Dialog.Trigger asChild>
          <Button>Open composed dialog</Button>
        </Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Header>
            <Dialog.Title>Composed controls</Dialog.Title>
            <Dialog.Close />
          </Dialog.Header>
          <Dialog.Body>
            <p>Close with Escape or the button. Outside clicks keep this dialog open.</p>
          </Dialog.Body>
          <Dialog.Footer>
            <Dialog.Close asChild>
              <Button>Done</Button>
            </Dialog.Close>
          </Dialog.Footer>
        </Dialog.Content>
      </Dialog.Root>
      <Popup.Root>
        <Popup.Trigger asChild>
          <Button>Composed menu</Button>
        </Popup.Trigger>
        <Popup.Content aria-label="Composed actions">
          <Popup.Item onSelect={() => setClicks((count) => count + 1)}>Count selection</Popup.Item>
          <Popup.Item asChild>
            <button type="button">Custom menu item</button>
          </Popup.Item>
        </Popup.Content>
      </Popup.Root>
      <Button asChild disabled>
        <a href="#disabled" onClick={() => setClicks((count) => count + 1)}>
          Disabled child link
        </a>
      </Button>
      <Dialog.Root>
        <Dialog.Trigger asChild>
          <Button onClick={(event) => event.preventDefault()}>Cancel opening</Button>
        </Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Title>This dialog stays closed</Dialog.Title>
        </Dialog.Content>
      </Dialog.Root>
      <output aria-live="polite">Selections: {clicks}</output>
    </div>
  )
}
export const ChildComposition: Story = { render: () => <ChildCompositionDemo /> }
