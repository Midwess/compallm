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
              <ul className="divide-y divide-divider rounded-lg bg-inset px-4">
                {items.map((item) => (
                  <li key={item.id} className="py-4 text-xsm">
                    {item.name}
                  </li>
                ))}
              </ul>
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
