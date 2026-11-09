import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { Popup } from '../../../components/popup'
import { PageHeader, Title, Functions } from '../../../components/page-header'
import { ProviderChooser } from './chooser'
import { ProviderConfiguration } from './configuration'
import { ModelDialog } from './model-dialog'
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
  const name = useProvider((state) => (state.intent === 'edit' ? state.savedProvider?.name : state.name))
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
          className="cursor-pointer text-muted hover:text-text focus-visible:outline-focus"
        >
          Create provider
        </button>
        <span aria-hidden="true" className="text-muted">
          /
        </span>
        <span>{step === 'choose' ? 'Choose provider' : 'Configure'}</span>
      </h1>
    </PageHeader>
  )
}
function ProviderActions() {
  const controller = useController()
  return (
    <Popup>
      <Popup.Trigger aria-label="Provider actions">
        <DotsThreeIcon aria-hidden="true" className="size-4" />
      </Popup.Trigger>
      <Popup.Content aria-label="Provider actions">
        <Popup.Item onClick={controller.onCloneProvider}>Clone provider</Popup.Item>
      </Popup.Content>
    </Popup>
  )
}
function ProviderContent() {
  const step = useProvider((state) => state.step)
  return step === 'choose' ? <ProviderChooser /> : <ProviderConfiguration />
}
function ProviderDialog() {
  const open = useProvider((state) => state.modelDraft !== null)
  return open ? <ModelDialog /> : null
}
export function CreateProviderPage() {
  return (
    <>
      <ProviderHeader />
      <ProviderContent />
      <ProviderDialog />
    </>
  )
}

export { canCreateProvider, canSaveProvider, hasValidModelPrices, isValidModelDraft, providerSnapshot } from './validation'

export { capabilityOptions } from './model-options'

export { providerCreateRequest, providerReplaceRequest } from './contract'
