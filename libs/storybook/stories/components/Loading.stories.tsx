import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { LoadingRegion, Skeleton } from '../../src/components/loading'
import { FormField } from '../../src/components/form-field'
import { SectionSurface } from '../../src/components/surfaces'
import { Button } from '../../src/components/buttons'
import { Table } from '../../src/components/table'

const meta = { title: 'Components/Loading', component: Example } satisfies Meta<typeof Example>
export default meta
type Story = StoryObj<typeof meta>

function Example({
  ready = false,
  pending = true,
  error = false,
  empty = false,
}: {
  ready?: boolean
  pending?: boolean
  error?: boolean
  empty?: boolean
}) {
  const [loaded, setLoaded] = useState(ready)
  return (
    <div className="mx-auto grid max-w-xl gap-4 p-6">
      <h1 className="text-2xl font-medium">Connection</h1>
      <Button onClick={() => setLoaded((value) => !value)}>{loaded ? 'Show first load' : 'Show loaded content'}</Button>
      {error ? (
        <p role="alert">
          Could not load the connection. <Button onClick={() => setLoaded(true)}>Retry</Button>
        </p>
      ) : null}
      <LoadingRegion ready={loaded} pending={pending && !error} label="Loading connection">
        <SectionSurface className="grid gap-4">
          <FormField.Root>
            <FormField.Label>Name</FormField.Label>
            <FormField.Input defaultValue={loaded ? 'My connection' : ''} placeholder="Enter connection name" />
          </FormField.Root>
          <Skeleton>
            <p className="h-5 text-sm">Connection details</p>
          </Skeleton>
          <Table.Root>
            <Table.Content columns={[{ id: 'model' }, { id: 'status' }]}>
              <Table.Header>
                <Table.Row>
                  <Table.Head>Model</Table.Head>
                  <Table.Head>Status</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body items={loaded && !empty ? [{ id: 'model', name: 'Example model' }] : []} getKey={(item) => item.id}>
                {(item) => (
                  <Table.Row>
                    <Table.Cell>{item.name}</Table.Cell>
                    <Table.Cell>Enabled</Table.Cell>
                  </Table.Row>
                )}
              </Table.Body>
            </Table.Content>
          </Table.Root>
          {loaded && empty ? <p className="text-sm text-muted-foreground">No models available.</p> : null}
        </SectionSurface>
      </LoadingRegion>
    </div>
  )
}
export const FirstLoad: Story = { render: () => <Example /> }
export const Loaded: Story = { render: () => <Example ready pending={false} /> }
export const Refreshing: Story = { render: () => <Example ready /> }
export const RecoverableError: Story = { render: () => <Example error /> }

export const Empty: Story = { render: () => <Example ready pending={false} empty /> }
