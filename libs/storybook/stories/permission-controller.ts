import {
  createPermissionStore,
  createPermissionAbility,
  permissionKey,
  publishPermissions,
  type PermissionController,
  type PermissionCheck,
  type PermissionStatus,
} from '../src/permission'
export class PreviewPermissionController implements PermissionController {
  readonly store = createPermissionStore()
  readonly ability = createPermissionAbility()
  constructor(private readonly decide: (check: PermissionCheck) => PermissionStatus = () => 'allowed') {}
  ensure = async (checks: PermissionCheck[]) => {
    const entries = { ...this.store.getState().entries }
    for (const check of checks) {
      const key = permissionKey(check)
      if (!entries[key]) entries[key] = { check, status: this.decide(check) }
    }
    publishPermissions(this, entries)
  }
  retry = async (checks: PermissionCheck[]) => {
    const entries = { ...this.store.getState().entries }
    for (const check of checks) delete entries[permissionKey(check)]
    publishPermissions(this, entries)
    await this.ensure(checks)
  }
}
