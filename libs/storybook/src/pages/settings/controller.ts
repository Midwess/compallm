import { useContext } from 'react'
import { useFormStore } from '../../components/form'
import { SettingsControllerContext } from '../../context'
import type { SettingsState } from './types'

export function useSettingsController() {
  const controller = useContext(SettingsControllerContext)
  if (!controller) throw new Error('SettingsPage requires SettingsControllerContext.')
  return controller
}
export function useSettings<T>(selector: (state: SettingsState) => T, names: readonly string[] = []) {
  return useFormStore(useSettingsController().store, selector, names)
}

export { useCodeAppearance } from './preferences/user/controller'
