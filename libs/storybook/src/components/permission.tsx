import { useContext, useEffect, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { PermissionControllerContext } from '../context'
import { permissionChecks, permissionKey, type PermissionRequestInput, type PermissionStatus } from '../permission'

export function usePermissionController() {
  const controller = useContext(PermissionControllerContext)
  if (!controller) throw new Error('PermissionControllerContext is required inside an organization workspace.')
  return controller
}
export function usePermissionChecks(
  checks: ReturnType<typeof permissionChecks>,
  facts: import('../permission').PermissionFacts[] = [],
) {
  const controller = usePermissionController()
  const serialized = JSON.stringify(checks)
  const serializedFacts = JSON.stringify(facts)
  useEffect(() => {
    controller.setFacts?.(JSON.parse(serializedFacts))
  }, [controller, serializedFacts])
  const statuses = useStore(controller.store, (state) =>
    checks.map((check) => state.entries[permissionKey(check)]?.status ?? 'unchecked').join(','),
  )
  useEffect(() => {
    void controller.evaluate(JSON.parse(serialized))
  }, [controller, serialized, statuses])
  return checks.map((check, index) => {
    const status = statuses.split(',')[index]
    return status === 'unchecked' ? 'denied' : (status as PermissionStatus)
  })
}
export function usePermission(request: PermissionRequestInput) {
  const checks = permissionChecks(request)
  const values = usePermissionChecks(checks)
  const allowed =
    checks.length > 0 && (request.match === 'any' ? values.includes('allowed') : values.every((status) => status === 'allowed'))
  const status: PermissionStatus = allowed
    ? 'allowed'
    : request.match !== 'any' && values.includes('denied')
      ? 'denied'
      : values.includes('error')
        ? 'error'
        : 'denied'
  return { allowed, status, disabled: !allowed }
}
export interface PermissionRequestProps extends PermissionRequestInput {
  children: (allowed: boolean) => ReactNode
}
export function PermissionRequest({ children, ...request }: PermissionRequestProps) {
  return children(usePermission(request).allowed)
}
