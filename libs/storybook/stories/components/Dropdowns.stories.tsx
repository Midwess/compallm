import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import * as Menus from '../../src/components/dropdowns'
import { Preview, Sample } from './Preview'
import { Button } from '../../src/components/buttons'
import { Dialog } from '../../src/components/dialog'

const options = [
  { value: 'typescript', label: 'TypeScript' },
  { value: 'rust', label: 'Rust' },
  { value: 'python', label: 'Python' },
  { value: 'unavailable', label: 'Unavailable', disabled: true },
]
const meta = { title: 'Components/Dropdowns', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function ControlledDropdown() {
  const [value, setValue] = useState('typescript')
  return (
    <>
      <Menus.Dropdown label="Language" options={options} value={value} onValueChange={setValue} />
      <output aria-label="Selected language">{value}</output>
    </>
  )
}

function ComposedSelect() {
  const [value, setValue] = useState<string | null>('rust')
  const [visible, setVisible] = useState(true)
  return (
    <Preview>
      <Sample label="Shared select parts">
        {visible ? (
          <Menus.Select.Root items={options} value={value} onValueChange={setValue}>
            <Menus.Select.Trigger aria-label="Composed language" className="w-60">
              <Menus.Select.Value />
            </Menus.Select.Trigger>
            <Menus.Select.Content aria-label="Composed language">
              {options.map((item) => (
                <Menus.Select.Item key={item.value} value={item.value} disabled={item.disabled}>
                  {item.label}
                </Menus.Select.Item>
              ))}
            </Menus.Select.Content>
          </Menus.Select.Root>
        ) : null}
        <output aria-label="Composed selection">{value}</output>
        <button type="button" className="text-xs text-primary" onClick={() => setVisible((current) => !current)}>
          {visible ? 'Remove select' : 'Restore select'}
        </button>
      </Sample>
    </Preview>
  )
}

export const SelectComposition: Story = { render: () => <ComposedSelect /> }

function DialogSelect() {
  const [open, setOpen] = useState(false)
  return (
    <Preview>
      <Button onClick={() => setOpen(true)}>Open select dialog</Button>
      {open ? (
        <Dialog.Root
          open
          onOpenChange={(open) => {
            if (!open) setOpen(false)
          }}
        >
          <Dialog.Content>
            <Dialog.Header>
              <Dialog.Title>Choose language</Dialog.Title>
              <Dialog.Close />
            </Dialog.Header>
            <Dialog.Body>
              <Menus.Select.Control aria-label="Dialog language" options={options} defaultValue="rust" />
            </Dialog.Body>
            <Dialog.Footer>
              <Dialog.Cancel />
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog.Root>
      ) : null}
    </Preview>
  )
}

export const SelectInDialog: Story = { render: () => <DialogSelect /> }
export const Select: Story = {
  render: () => (
    <Preview>
      <Sample label="Custom select">
        <Menus.Select.Control aria-label="Language" options={options} />
      </Sample>
      <Sample label="Options with icons">
        <Menus.Select.Control
          aria-label="Badge style"
          options={[
            {
              value: 'dot',
              label: 'Dot',
              icon: <span aria-hidden="true" className="size-1.5 rounded-full bg-muted-foreground" />,
            },
            {
              value: 'count',
              label: 'Count',
              icon: (
                <span aria-hidden="true" className="text-muted-foreground">
                  1
                </span>
              ),
            },
          ]}
        />
      </Sample>
      <Sample label="Disabled">
        <Menus.Select.Control aria-label="Disabled language" options={options} disabled />
      </Sample>
    </Preview>
  ),
}
export const Dropdown: Story = {
  render: () => (
    <Preview>
      <Sample label="Keyboard and pointer selection">
        <ControlledDropdown />
      </Sample>
      <Sample label="Expanded">
        <Menus.Dropdown label="Expanded language" options={options} defaultOpen />
      </Sample>
      <Sample label="Disabled">
        <Menus.Dropdown label="Disabled language" options={options} disabled />
      </Sample>
    </Preview>
  ),
}
export const ThemeMenu: Story = {
  render: () => (
    <Preview>
      <Sample label="Seven theme choices">
        <Menus.ThemeMenu />
      </Sample>
    </Preview>
  ),
}
export const DiagramMenu: Story = {
  render: () => (
    <Preview>
      <Sample label="Four diagram styles">
        <Menus.DiagramMenu />
      </Sample>
    </Preview>
  ),
}
export const DiagramDropdown: Story = {
  render: () => (
    <Preview>
      <Sample label="Diagram style">
        <Menus.DiagramDropdown defaultValue="Hairline" />
      </Sample>
    </Preview>
  ),
}
export const MenuGroupHeading: Story = {
  render: () => (
    <Preview>
      <Sample label="Menu group">
        <Menus.MenuGroupHeading>Integrations</Menus.MenuGroupHeading>
      </Sample>
    </Preview>
  ),
}
export const SelectOption: Story = {
  render: () => (
    <Preview>
      <Sample label="Default and selected">
        <Menus.Select.Root
          defaultValue="selected"
          items={[
            { value: 'default', label: 'Default option' },
            { value: 'selected', label: 'Selected option' },
          ]}
        >
          <Menus.Select.Trigger aria-label="Option selection" className="w-60">
            <Menus.Select.Value />
          </Menus.Select.Trigger>
          <Menus.Select.Content aria-label="Option selection">
            <Menus.SelectOption value="default">Default option</Menus.SelectOption>
            <Menus.SelectOption value="selected">Selected option</Menus.SelectOption>
          </Menus.Select.Content>
        </Menus.Select.Root>
      </Sample>
    </Preview>
  ),
}
