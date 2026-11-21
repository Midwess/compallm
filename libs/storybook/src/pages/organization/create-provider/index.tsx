import { CatalogStatus } from './catalog-status'
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
  const templatePreview = useProvider((state) => state.templatePreview)
  const step = useProvider((state) => state.step)
  const intent = useProvider((state) => state.intent)
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
        <Functions>
          <ProviderActions />
        </Functions>
      </PageHeader>
    )
  return (
    <PageHeader>
      <h1 className="flex min-w-0 items-center gap-6 text-xs leading-5 font-normal">
        <button
          type="button"
          onClick={controller.onChooseProvider}
          className="cursor-pointer text-muted-foreground hover:text-foreground focus-visible:outline-ring"
        >
          {templatePreview ? 'Provider templates' : 'Create provider'}
        </button>
        <span aria-hidden="true" className="text-muted-foreground">
          /
        </span>
        <span>{templatePreview ? 'Configure template' : 'Configure'}</span>
      </h1>
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
  const modelId = useProvider((state) => state.modelDraft?.id, ['modelDraft.id'])
  const templatePreview = useProvider((state) => state.templatePreview)
  const loading = useProvider((state) => state.modelLoading || !!state.catalogError)
  const content = (
    <>
      <div className={modelId ? 'hidden' : 'contents'} inert={!!modelId || undefined}>
        {!modelId ? <ProviderHeader /> : null}
        <ProviderContent />
      </div>
      {modelId ? templatePreview && loading ? <CatalogStatus /> : <ModelSettingsPage key={modelId} /> : null}
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
