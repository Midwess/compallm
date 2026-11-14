import { PermissionRequest } from '../../../components/permission'
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
  const name = useProvider((state) => (state.intent === 'edit' ? state.savedProvider?.name : state.name), ['name'])
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
          Create provider
        </button>
        <span aria-hidden="true" className="text-muted-foreground">
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
    <Popup.Root>
      <Popup.Trigger aria-label="Provider actions">
        <DotsThreeIcon aria-hidden="true" className="size-4" />
      </Popup.Trigger>
      <Popup.Content aria-label="Provider actions">
        <PermissionRequest actions={['PROVIDER_CREATE']}>
          <Popup.Item onSelect={controller.onCloneProvider}>Clone provider</Popup.Item>
        </PermissionRequest>
      </Popup.Content>
    </Popup.Root>
  )
}
function ProviderContent() {
  const step = useProvider((state) => state.step)
  return step === 'choose' ? <ProviderChooser /> : <ProviderConfiguration />
}
function ProviderDialog() {
  const open = useProvider((state) => state.modelDraft !== null, ['modelDraft'])
  return open ? <ModelDialog /> : null
}
export function CreateProviderPage() {
  const intent = useProvider((state) => state.intent)
  return (
    <PermissionRequest actions={intent === 'edit' ? ['PROVIDER_READ'] : ['PROVIDER_CREATE']} restricted="fallback">
      <ProviderHeader />
      <ProviderContent />
      <ProviderDialog />
    </PermissionRequest>
  )
}

export { canCreateProvider, canSaveProvider, hasValidModelPrices, isValidModelDraft, providerSnapshot } from './validation'

export { capabilityOptions } from './model-options'

export { providerCreateRequest, providerReplaceRequest } from './contract'
