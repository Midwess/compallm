import { useContext } from 'react'
import { useFormStore } from '../../../components/form'
import { CreateProviderControllerContext } from '../../../context'
import type { CreateProviderState } from './types'

export function useController() {
  const controller = useContext(CreateProviderControllerContext)
  if (!controller) throw new Error('CreateProviderPage requires CreateProviderControllerContext.')
  return controller
}
export function useProvider<T>(selector: (state: CreateProviderState) => T, names: readonly string[] = []) {
  return useFormStore(useController().store, selector, names)
}
