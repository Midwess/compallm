import { untrack } from 'svelte'
import { getPermissionController } from '../context'
import {
  permissionChecks,
  permissionKey,
  type PermissionCheck,
  type PermissionFacts,
  type PermissionRequestInput,
  type PermissionStatus,
} from '../permission'
export function usePermissionChecks(read: () => PermissionCheck[], facts: () => PermissionFacts[] = () => []) {
  const controller = getPermissionController()
  const checks = $derived(read())
  const serializedFacts = $derived(JSON.stringify(facts()))
  const serializedChecks = $derived(JSON.stringify(checks))
  const statuses = $derived(
    checks.map((check) => controller.store.getState().entries[permissionKey(check)]?.status ?? 'unchecked').join(','),
  )
  $effect(() => {
    const snapshot = serializedFacts
    untrack(() => controller.setFacts?.(JSON.parse(snapshot)))
  })
  $effect(() => {
    const snapshot = serializedChecks
    void statuses
    untrack(() => {
      void controller.evaluate(JSON.parse(snapshot))
    })
  })
  return {
    get current(): PermissionStatus[] {
      return checks.map((check) => controller.store.getState().entries[permissionKey(check)]?.status ?? 'denied')
    },
    get pending() {
      return checks.some((check) => !controller.store.getState().entries[permissionKey(check)])
    },
  }
}
export function usePermission(read: () => PermissionRequestInput) {
  const checks = usePermissionChecks(() => permissionChecks(read()))
  return {
    get allowed() {
      return (
        checks.current.length > 0 &&
        (read().match === 'any' ? checks.current.includes('allowed') : checks.current.every((status) => status === 'allowed'))
      )
    },
    get disabled() {
      return !this.allowed
    },
    get pending() {
      return checks.pending
    },
    get status(): PermissionStatus {
      return this.allowed ? 'allowed' : checks.current.includes('error') ? 'error' : 'denied'
    },
  }
}
