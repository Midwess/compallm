import { List } from '../../components/list'
import { PermissionRequest } from '../../components/permission'
import { useContext } from 'react'
import { useStore } from 'zustand'
import { OrganizationControllerContext } from '../../context'
import { Button } from '../../components/buttons'

export function OrganizationCatalogPage({ kind }: { kind: 'models' | 'providers' }) {
  const controller = useContext(OrganizationControllerContext)
  if (!controller) throw new Error('Organization catalog requires OrganizationControllerContext.')
  const items = useStore(controller.store, (state) => state[kind])
  const providers = kind === 'providers'
  return (
    <PermissionRequest actions={[providers ? 'provider.read' : 'model.read']}>
      {(allowed) =>
        allowed ? (
          <section className="mx-auto grid max-w-4xl gap-8 px-8 py-10">
            <header className="flex items-center justify-between gap-4">
              <h1 className="text-2xl font-medium">{providers ? 'Providers' : 'Models'}</h1>
              {providers ? (
                <PermissionRequest actions={['provider.create']}>
                  {(allowed) => (allowed ? <Button onClick={controller.onCreateProvider}>Add provider</Button> : null)}
                </PermissionRequest>
              ) : null}
            </header>
            {items.length ? (
              <List.Root>
                <List.Items items={items} getKey={(item) => item.id} className="rounded-lg bg-inset px-4">
                  {(item) => <div className="border-b border-divider px-4 py-4 text-xsm">{item.name}</div>}
                </List.Items>
              </List.Root>
            ) : (
              <div className="grid gap-2 rounded-lg bg-inset px-6 py-8">
                <h2 className="text-sm font-medium">{providers ? 'No providers available' : 'No models available'}</h2>
                <p className="text-xsm text-muted-foreground">
                  {providers ? 'Providers will appear here when available.' : 'Models will appear here when available.'}
                </p>
              </div>
            )}
          </section>
        ) : (
          <p role="status">You do not have access to this page.</p>
        )
      }
    </PermissionRequest>
  )
}
