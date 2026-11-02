import { useContext } from 'react'
import { useStore } from 'zustand'
import { CreateProviderControllerContext } from '../../../context'
import type { CreateProviderState } from './types'

export function useController() {
  const controller = useContext(CreateProviderControllerContext)
  if (!controller) throw new Error('CreateProviderPage requires CreateProviderControllerContext.')
  return controller
}
export function useProvider<T>(selector: (state: CreateProviderState) => T) {
  return useStore(useController().store, selector)
}
