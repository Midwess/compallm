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
        <Dialog onClose={() => setOpen(false)}>
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title>Edit details</Dialog.Title>
              <Dialog.Close />
            </Dialog.Header>
            <Dialog.Discard />
            <Dialog.Body>
              <FormField>
                <FormField.Label>Name</FormField.Label>
                <FormField.Input autoFocus placeholder="Enter name" />
              </FormField>
            </Dialog.Body>
            <Dialog.Footer>
              <Dialog.Cancel />
              <Button variant="primary" onClick={() => setOpen(false)}>
                Save
              </Button>
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog>
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
        <AlertDialog onClose={() => setOpen(false)}>
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
        </AlertDialog>
      ) : null}
    </>
  )
}
export const DialogComposition: Story = { render: () => <DialogDemo /> }
export const AlertComposition: Story = { render: () => <AlertDemo /> }
export const PopupComposition: Story = {
  render: () => (
    <Popup>
      <Popup.Trigger aria-label="Open actions">Actions</Popup.Trigger>
      <Popup.Content aria-label="Actions">
        <Popup.Item>Clone</Popup.Item>
        <Popup.Item>Settings</Popup.Item>
      </Popup.Content>
    </Popup>
  ),
}
export const TableComposition: Story = {
  render: () => (
    <Table>
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
    </Table>
  ),
}
export const CategorizedTable: Story = {
  render: () => (
    <div className="max-h-64 max-w-160 overflow-auto rounded-lg bg-inset p-4">
      <Table className="overflow-visible">
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
      </Table>
    </div>
  ),
}
export const ControlsComposition: Story = {
  render: () => (
    <div className="grid max-w-md gap-4">
      <FormField>
        <FormField.Label>Connection name</FormField.Label>
        <FormField.Input placeholder="Enter name" />
        <FormField.Hint>A name for this connection.</FormField.Hint>
      </FormField>
      <Searchbox>
        <Searchbox.Input aria-label="Search models" placeholder="Search models…" />
      </Searchbox>
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

export const FilterSubmenus: Story = {
  render: () => (
    <Popup>
      <Popup.IconTrigger aria-label="Filter example">
        <FunnelIcon aria-hidden="true" className="size-3.5" />
      </Popup.IconTrigger>
      <Popup.Content aria-label="Filter example">
        <Popup.Submenu label="Enabled">
          <Popup.Item>All</Popup.Item>
          <Popup.Item>Enabled</Popup.Item>
          <Popup.Item>Disabled</Popup.Item>
        </Popup.Submenu>
        <Popup.Submenu label="Model type">
          <Popup.Item>OpenAI Completion</Popup.Item>
          <Popup.Item>OpenAI Response</Popup.Item>
          <Popup.Item>Anthropic</Popup.Item>
        </Popup.Submenu>
        <Popup.Separator />
        <Popup.Item>Clear all filters</Popup.Item>
      </Popup.Content>
    </Popup>
  ),
}
