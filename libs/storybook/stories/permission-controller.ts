import {
  createPermissionStore,
  permissionKey,
  publishPermissions,
  type PermissionController,
  type PermissionCheck,
  type PermissionStatus,
} from '../src/permission'
export class PreviewPermissionController implements PermissionController {
  readonly store = createPermissionStore()
  constructor(private readonly decide: (check: PermissionCheck) => PermissionStatus = () => 'allowed') {}
  evaluate = async (checks: PermissionCheck[]) => {
    const entries = { ...this.store.getState().entries }
    for (const check of checks) {
      const key = permissionKey(check)
      if (!entries[key]) entries[key] = { check, status: this.decide(check) }
    }
    publishPermissions(this, entries)
  }
}
