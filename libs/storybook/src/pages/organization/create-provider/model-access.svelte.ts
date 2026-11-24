import { getCreateProviderController } from '../../../context'
import { usePermission } from '../../../components/permission'
export function useModelEditAccess() {
  const controller = getCreateProviderController()
  const state = $derived(controller.store.getState())
  const permission = usePermission(() => ({
    actions: [
      state.intent === 'edit'
        ? state.editingModelId
          ? 'provider.model.configuration.update'
          : 'provider.model.create'
        : 'provider.create',
    ],
    resource:
      state.intent === 'edit'
        ? state.editingModelId
          ? { type: 'ProviderModel', id: state.editingModelId }
          : { type: 'Provider', id: state.savedProvider?.id }
        : undefined,
    enabled: !state.templatePreview && (state.intent !== 'edit' || !!state.savedProvider?.id),
  }))
  return () => (state.templatePreview ? !!state.modelLoaded : permission.allowed)
}
