import { useContext, useEffect, type ReactElement, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { PermissionControllerContext } from '../context'
import { permissionChecks, permissionKey, type PermissionRequestInput, type PermissionStatus } from '../permission'
import { Slot } from './composition'
import { DisabledTooltip } from './feedback'

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
  restricted?: 'hidden' | 'disabled' | 'fallback'
  explanation?: string
  fallback?: ReactNode
  asChild?: boolean
  children: ReactNode | ((permission: ReturnType<typeof usePermission>) => ReactNode)
}
export function PermissionRequest({
  restricted = 'hidden',
  explanation = 'You do not have permission to perform this action.',
  fallback,
  asChild = false,
  children,
  ...request
}: PermissionRequestProps) {
  const permission = usePermission(request)
  if (restricted === 'disabled') {
    const content =
      typeof children === 'function' ? (
        children(permission)
      ) : asChild ? (
        <Slot disabled={permission.disabled}>{children as ReactElement}</Slot>
      ) : null
    if (content === null) throw new Error('Disabled PermissionRequest requires asChild or function children.')
    return (
      <DisabledTooltip disabled={permission.disabled} reason={explanation}>
        {content}
      </DisabledTooltip>
    )
  }
  if (permission.allowed) return typeof children === 'function' ? children(permission) : children
  if (restricted === 'hidden') return null
  return fallback ?? <p role="status">You do not have access to this page.</p>
}
