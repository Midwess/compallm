<script lang="ts">
  import { DisabledTooltip } from '../../../components/feedback'
  import { PermissionRequest, usePermission } from '../../../components/permission'

  import { FormError } from '../../../components/form'

  import { ProviderFieldsConstraints, ProviderHeaderConstraints } from '@compallm/contract/provider'
  import EyeIcon from 'phosphor-svelte/lib/EyeIcon'
  import EyeSlashIcon from 'phosphor-svelte/lib/EyeSlashIcon'

  import { SectionSurface } from '../../../components/surfaces'
  import { FormField } from '../../../components/form-field'

  import { KeyValueEditor } from '../../../components/key-value-editor'
  import { ProviderAvatar } from '../../../components/provider-avatar'
  import { FieldHelp } from '../../../components/feedback'

  import { acceptsHeaderInput } from './validation'
  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'

  const __provider = getCreateProviderController()
  let keyVisible = $state(false)
  const controller = untrack(() => getCreateProviderController())
  let editing = $derived(readStore(__provider.store, (state) => state.intent === 'edit'))
  let preview = $derived(readStore(__provider.store, (state) => state.templatePreview))
  let submitting = $derived(readStore(__provider.store, (state) => state.submitting))
  let providerId = $derived(readStore(__provider.store, (state) => state.savedProvider?.id))
  const edit = usePermission(() => ({
    actions: ['provider.configuration.update'],
    resource: { type: 'Provider', id: providerId },
    enabled: editing && !!providerId,
  }))
  let loading = $derived(readStore(__provider.store, (s) => s.providerLoading && !s.providerLoaded))
  let readOnly = $derived(readStore(__provider.store, (state) => state.readOnly))
  let busy = $derived(!!readOnly || submitting || !!loading || (editing && !edit.allowed))
  let selected = $derived(readStore(__provider.store, (state) => state.selectedProviderId))
  let avatarUrl = $derived(readStore(__provider.store, (state) => state.avatarUrl, ['avatarUrl']))
</script>

<SectionSurface class="grid min-w-0 gap-4">
  <fieldset disabled={busy} class="grid min-w-0 gap-4">
    <div class="flex min-w-0 items-end gap-3">
      <FormField.Root class="flex-1">
        <FormField.Label>Connection name</FormField.Label><FormField.Input
          name="name"
          required
          placeholder="Enter connection name"
        ></FormField.Input><FormError name="name"></FormError>
      </FormField.Root><ProviderAvatar name="avatarFile" provider={selected} src={avatarUrl} disabled={busy}
      ></ProviderAvatar><FormError name="avatarFile"></FormError>
    </div>
    <FormField.Root>
      <FormField.Label>Base URL</FormField.Label><FormField.Input
        name="baseUrl"
        type="url"
        required
        autocapitalize="none"
        spellcheck={false}
        placeholder="https://api.example.com/v1"
      ></FormField.Input><FormError name="baseUrl"></FormError>
    </FormField.Root>{#if !readOnly}<FormField.Root>
        <div class="flex items-center gap-1">
          <FormField.Label>API key</FormField.Label><FieldHelp label="About API keys">
            An API key is a secret credential from your provider that lets Compallm send requests to its models on your behalf.
            Create one in your provider’s dashboard and keep it private. Up to {ProviderFieldsConstraints.apiKey.maxLength} characters,
            without whitespace. When editing, leave this field empty to keep the current key.
          </FieldHelp>
        </div>
        <div class="relative min-w-0">
          <PermissionRequest
            actions={[editing ? 'provider.credential.replace' : 'provider.create']}
            resource={editing ? { type: 'Provider', id: providerId } : undefined}
            enabled={!editing || !!providerId}
          >
            {#snippet children(allowed)}<DisabledTooltip
                disabled={!allowed && !preview}
                reason="You do not have permission to perform this action."
              >
                <FormField.Input
                  disabled={!allowed && !preview}
                  type={keyVisible ? 'text' : 'password'}
                  class="pr-9"
                  autocomplete="off"
                  required={!editing}
                  autocapitalize="none"
                  spellcheck={false}
                  name="apiKey"
                  placeholder={editing ? 'Leave empty to keep current key' : 'Enter your API key'}
                ></FormField.Input>
              </DisabledTooltip>{/snippet}
          </PermissionRequest><FormError name="apiKey"></FormError><button
            type="button"
            aria-label={keyVisible ? 'Hide API key' : 'Show API key'}
            aria-pressed={keyVisible}
            class={`
absolute inset-y-0 right-0 flex w-8 items-center justify-center rounded-lg text-muted-foreground
hover:text-foreground focus-visible:outline-muted-foreground
`}
            onclick={() => (keyVisible = ((visible) => !visible)(keyVisible))}
          >
            {#if keyVisible}<EyeSlashIcon aria-hidden="true" class="size-4"></EyeSlashIcon>{:else}<EyeIcon
                aria-hidden="true"
                class="size-4"
              ></EyeIcon>{/if}
          </button>
        </div>
      </FormField.Root>{:else}{/if}
  </fieldset>
  <KeyValueEditor.Root
    name="headers"
    acceptEntry={acceptsHeaderInput}
    keyMaxLength={ProviderHeaderConstraints.key.maxLength}
    disabled={busy || !controller.onHeadersChanged}
  >
    <KeyValueEditor.Disclosure>Headers</KeyValueEditor.Disclosure>
  </KeyValueEditor.Root><FormError name="headers"></FormError><FormError name="models"></FormError>
</SectionSurface>
