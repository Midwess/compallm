import { Profiler, useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { Table } from '../../src/components/table'
import { List } from '../../src/components/list'
import { Form, useManagedForm } from '../../src/components/form'
import { TextField } from '../../src/components/fields'
import { Button } from '../../src/components/buttons'
import { CollectionForms } from '../../src/components/collection-forms'
import { ChoicePicker } from '../../src/components/choice-picker'
import { PageProgress } from '../../src/components/page-progress'

const records = Array.from({ length: 1000 }, (_, index) => ({ id: `row-${index}`, name: `Record ${index + 1}` }))
function Editable({ record }: { record: (typeof records)[number] }) {
  const form = useManagedForm({ name: record.name }, { submit: async (values) => values })
  const [open, setOpen] = useState(false)
  const [activated, setActivated] = useState(false)
  return (
    <Form.Root model={form}>
      <TextField name="name" aria-label={`Edit ${record.id}`} />
      <Button type="submit">Save</Button>
      {activated ? (
        <ChoicePicker.Root
          open={open}
          onOpenChange={setOpen}
          options={[{ id: 'group', label: 'Example group' }]}
          value={[]}
          onValueChange={() => {}}
        >
          <ChoicePicker.Trigger aria-label={`Groups ${record.id}`}>Groups</ChoicePicker.Trigger>
          {open ? (
            <ChoicePicker.Content align="start">
              <ChoicePicker.List>{(option) => <ChoicePicker.Item key={option.id} value={option} />}</ChoicePicker.List>
            </ChoicePicker.Content>
          ) : null}
        </ChoicePicker.Root>
      ) : (
        <Button
          aria-label={`Groups ${record.id}`}
          onClick={() => {
            setActivated(true)
            setOpen(true)
          }}
        >
          Groups
        </Button>
      )}
    </Form.Root>
  )
}
function Demo({ grouped = false, variable = false }: { grouped?: boolean; variable?: boolean }) {
  const [forms] = useState(() => new CollectionForms())
  const [items, setItems] = useState(records)
  const [active, setActive] = useState(false)
  useEffect(() => forms.dispose, [forms])
  return (
    <div className="h-dvh overflow-auto bg-surface p-6 text-foreground">
      <PageProgress active={active} />
      <div className="mb-4 flex gap-2">
        <Button
          onClick={() => {
            setActive(true)
            setTimeout(() => {
              setItems(records.slice(0, 100))
              setActive(false)
            }, 200)
          }}
        >
          Show 100
        </Button>
        <Button onClick={() => setItems(records)}>Show 1,000</Button>
        <Button onClick={() => setItems((rows) => rows.slice().reverse())}>Reverse order</Button>
      </div>
      <Profiler
        id="collection"
        onRender={(_id, phase, duration) => performance.mark('collection-render', { detail: { phase, duration } })}
      >
        <Table.Root forms={forms} className="overflow-visible">
          <Table.Content aria-label="Bounded editable records" columns={[{ id: 'name' }, { id: 'edit' }]}>
            <Table.Header className="sticky top-0 z-20 bg-surface">
              <Table.Row>
                <Table.Head>Name</Table.Head>
                <Table.Head>Edit</Table.Head>
              </Table.Row>
            </Table.Header>
            {grouped ? (
              ['First', 'Second'].map((label, index) => (
                <Table.Category
                  key={label}
                  label={label}
                  columns={2}
                  items={items.slice(index * 500, (index + 1) * 500)}
                  getKey={(row) => row.id}
                >
                  {(row) => (
                    <Table.Row>
                      <Table.Cell>{row.name}</Table.Cell>
                      <Table.Cell>
                        <Editable record={row} />
                      </Table.Cell>
                    </Table.Row>
                  )}
                </Table.Category>
              ))
            ) : (
              <Table.Body items={items} getKey={(row) => row.id} estimateSize={48}>
                {(row) => (
                  <Table.Row className={variable && Number(row.id.slice(4)) % 4 === 0 ? 'h-24' : undefined}>
                    <Table.Cell>{row.name}</Table.Cell>
                    <Table.Cell>
                      <Editable record={row} />
                    </Table.Cell>
                  </Table.Row>
                )}
              </Table.Body>
            )}
          </Table.Content>
        </Table.Root>
      </Profiler>
    </div>
  )
}
const meta = { title: 'Components/Collections', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const EditableTable: Story = { render: () => <Demo /> }
export const Categories: Story = { render: () => <Demo grouped /> }
export const VariableHeights: Story = { render: () => <Demo variable /> }
export const WindowList: Story = {
  render: () => (
    <div className="bg-surface p-6 text-foreground">
      <List.Root>
        <List.Items aria-label="Bounded window list" items={records} getKey={(row) => row.id}>
          {(row) => <div className="h-12 border-b border-divider px-4 py-3">{row.name}</div>}
        </List.Items>
      </List.Root>
    </div>
  ),
}
