import { useContext, useEffect, type ReactElement, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { PermissionControllerContext } from '../context'
import { permissionChecks, permissionKey, type PermissionRequestInput, type PermissionStatus } from '../permission'
import { Slot } from './composition'
import { DisabledTooltip } from './feedback'
import { Button } from './buttons'

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
    checks.map((check) => state.entries[permissionKey(check)]?.status ?? 'loading').join(','),
  )
  useEffect(() => {
    void controller.ensure(JSON.parse(serialized))
  }, [controller, serialized, statuses])
  return checks.map((check, index) => {
    const status = statuses.split(',')[index] as PermissionStatus
    return status
  })
}
export function usePermission(request: PermissionRequestInput) {
  const controller = usePermissionController()
  const checks = permissionChecks(request)
  const values = usePermissionChecks(checks)
  const allowed =
    checks.length > 0 && (request.match === 'any' ? values.includes('allowed') : values.every((status) => status === 'allowed'))
  const status: PermissionStatus = allowed
    ? 'allowed'
    : request.match !== 'any' && values.includes('denied')
      ? 'denied'
      : values.includes('loading')
        ? 'loading'
        : values.includes('error')
          ? 'error'
          : 'denied'
  return { allowed, status, disabled: !allowed, retry: () => controller.retry(checks) }
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
    const reason =
      permission.status === 'loading'
        ? 'Checking permissions…'
        : permission.status === 'error'
          ? 'Permissions could not be loaded.'
          : explanation
    return (
      <>
        <DisabledTooltip disabled={permission.disabled} reason={reason}>
          {content}
        </DisabledTooltip>
        {permission.status === 'error' ? (
          <Button
            onClick={() => {
              void permission.retry()
            }}
          >
            Retry permissions
          </Button>
        ) : null}
      </>
    )
  }
  if (permission.allowed) return typeof children === 'function' ? children(permission) : children
  if (restricted === 'hidden') return null
  if (permission.status === 'loading') return <p role="status">Checking permissions…</p>
  if (permission.status === 'error')
    return (
      <div role="alert">
        <p>Permissions could not be loaded.</p>
        <Button
          onClick={() => {
            void permission.retry()
          }}
        >
          Retry
        </Button>
      </div>
    )
  return fallback ?? <p role="status">You do not have access to this page.</p>
}

export function PermissionRecovery() {
  const controller = usePermissionController()
  const failed = useStore(controller.store, (state) => Object.values(state.entries).some((entry) => entry.status === 'error'))
  if (!failed) return null
  return (
    <div
      role="alert"
      className="fixed right-4 bottom-4 z-50 flex items-center gap-3 rounded-lg border border-border
bg-elevated p-3 text-xs text-foreground shadow-menu"
    >
      <span>Some permissions could not be loaded.</span>
      <Button
        onClick={() => {
          void controller.retry(
            Object.values(controller.store.getState().entries)
              .filter((entry) => entry.status === 'error')
              .map((entry) => entry.check),
          )
        }}
      >
        Retry permissions
      </Button>
    </div>
  )
}
