import { Button } from '../../../components/buttons'
import { useController, useProvider } from './controller'

export function CatalogStatus() {
  const controller = useController()
  const error = useProvider((s) => s.catalogError)
  return (
    <>
      {error ? (
        <div role="alert" className="flex items-center gap-3 px-4 text-sm">
          <span>{error}</span>
          <Button onClick={controller.onRetryCatalog}>Retry</Button>
        </div>
      ) : null}
    </>
  )
}
