import { PermissionRequest } from '../../../components/permission'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { Popup } from '../../../components/popup'
import { PageHeader, Title, Functions } from '../../../components/page-header'
import { ProviderChooser } from './chooser'
import { ProviderConfiguration, useProviderForm } from './configuration'
import { ModelSettingsPage } from './model-settings'
import { useController, useProvider } from './controller'

export { createCreateProviderStore } from './store'
export type {
  CreateProviderController,
  CreateProviderState,
  ProviderPreset,
  ProviderModel,
  ModelDraft,
  TextModelProtocol,
  ModelStatusFilter,
  ModelCapability,
  HeaderEntry,
  EffortMapping,
  ThinkingLevel,
  EndpointMode,
  ApiProtocol,
  ConnectionTest,
  ProviderSnapshot,
} from './types'

function ProviderHeader() {
  const controller = useController()
  const step = useProvider((state) => state.step)
  const intent = useProvider((state) => state.intent)
  const readOnly = useProvider((state) => state.readOnly)
  const name = useProvider((state) => (state.intent === 'edit' ? state.savedProvider?.name : state.name), ['name'])
  if (step === 'choose')
    return (
      <PageHeader>
        <Title>Choose provider</Title>
      </PageHeader>
    )
  if (intent !== 'create')
    return (
      <PageHeader>
        <Title>{name}</Title>
        {!readOnly ? (
          <Functions>
            <ProviderActions />
          </Functions>
        ) : null}
      </PageHeader>
    )
  return (
    <PageHeader>
      <Title className="flex items-center gap-6">
        <button
          type="button"
          onClick={controller.onChooseProvider}
          className="cursor-pointer text-muted-foreground hover:text-foreground focus-visible:outline-ring"
        >
          Choose provider
        </button>
        <span aria-hidden="true" className="text-muted-foreground">
          /
        </span>
        <span>Configure</span>
      </Title>
    </PageHeader>
  )
}
function ProviderActions() {
  const controller = useController()
  return (
    <Popup.Root>
      <Popup.Trigger aria-label="Provider actions">
        <DotsThreeIcon aria-hidden="true" className="size-4" />
      </Popup.Trigger>
      <Popup.Content aria-label="Provider actions">
        <PermissionRequest actions={['provider.create']}>
          {(allowed) => (allowed ? <Popup.Item onSelect={controller.onCloneProvider}>Clone provider</Popup.Item> : null)}
        </PermissionRequest>
      </Popup.Content>
    </Popup.Root>
  )
}
function ProviderContent() {
  const step = useProvider((state) => state.step)
  return step === 'choose' ? <ProviderChooser /> : <ProviderConfiguration />
}
export function CreateProviderPage() {
  useProviderForm()
  const intent = useProvider((state) => state.intent)
  const modelId = useProvider(
    (state) => (state.templatePreview ? state.editingModelId : state.modelDraft?.id),
    ['modelDraft.id'],
  )
  const templatePreview = useProvider((state) => state.templatePreview)
  const content = (
    <>
      <div className={modelId ? 'hidden' : 'contents'} inert={!!modelId || undefined}>
        {!modelId ? <ProviderHeader /> : null}
        <ProviderContent />
      </div>
      {modelId ? <ModelSettingsPage key={modelId} /> : null}
    </>
  )
  if (templatePreview) return content
  return (
    <PermissionRequest actions={intent === 'edit' ? ['provider.read'] : ['provider.create']}>
      {(allowed) => (allowed ? content : <p role="status">You do not have access to this page.</p>)}
    </PermissionRequest>
  )
}

export { canCreateProvider, canSaveProvider, hasValidModelPrices, isValidModelDraft, providerSnapshot } from './validation'

export { capabilityOptions } from './model-options'
export { initializeModelDraft, modelDraftFingerprint } from './model-options'

export { providerCreateRequest, providerReplaceRequest } from './contract'

export { templateCustomization } from './contract'
