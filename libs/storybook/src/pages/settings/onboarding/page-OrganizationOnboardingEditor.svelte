<script lang="ts">
  import { cloneFormValue } from '../../../components/form-model.svelte'
  import { LoadingRegion, Skeleton } from '../../../components/loading'
  import set from 'lodash-es/set.js'
  import get from 'lodash-es/get.js'
  import { Form } from '../../../components/form'
  import { OrganizationOnboardingSettingsInput } from '@compallm/contract/validation'
  import { Button } from '../../../components/buttons'
  import { TextField } from '../../../components/fields'
  import { Select } from '../../../components/dropdowns'
  import { DisabledTooltip } from '../../../components/feedback'
  import { Switch } from '../../../components/selection'
  import { usePermission } from '../../../components/permission'
  import PreviewBoundary from './PreviewBoundary.svelte'
  import { OnboardingPreviewController } from '../../onboarding'
  import { SettingsContent, Heading, Section, Rows, Row, Empty } from '../shared'
  import { untrack } from 'svelte'

  import { readStore, readForm } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'
  import { PreviewCanvas } from './page'

  const settings = untrack(() => getSettingsController())
  let controller = $derived(settings.pages.onboarding!)
  let status = $derived(readStore(controller.store, (s) => s.status))
  let saved = $derived(readStore(controller.store, (s) => s.saved))
  let ready = $derived(!!saved)
  let color = $derived(readForm<string | null>(controller.store.form, 'draft.artworkColor'))
  const permission = usePermission(() => ({ actions: ['organization.onboarding.update'] }))
  let editable = $derived(permission.allowed)
  let preview = $state(untrack(() => new OnboardingPreviewController()))
  $effect(() => {
    void [controller]
    return untrack(() => {
      void controller.load()
      return controller.cancel
    })
  })
  $effect(() => {
    void [controller, settings, ready]
    return untrack(() => {
      if (!ready) return
      const form = controller.store.form
      form.configure({
        mode: 'blur',
        fields: {
          'draft.requireFullName': { mode: 'change' },
          'draft.requireAvatar': { mode: 'change' },
          'draft.artworkColor': { mode: 'change' },
          'draft.destination': { mode: 'change' },
        },
        parse: (values, names) => {
          const candidate = cloneFormValue(controller.store.getState().saved ?? values.draft!)
          for (const name of names) set(candidate!, name.slice('draft.'.length), get(values, name))
          const parsed = OrganizationOnboardingSettingsInput.safeParse(candidate)
          return parsed.success
            ? { values }
            : { issues: parsed.error.issues.map((i) => ({ ...i, path: ['draft', ...i.path] })) }
        },
        error: (error, retry) => settings.apiError(error, retry),
        submit: async (_values, patch, signal) => ({ draft: await controller.update(patch.draft ?? {}, signal) }),
      })
      return () => form.stop()
    })
  })
  $effect(() => {
    void [saved, preview, settings]
    return untrack(() => {
      if (saved)
        preview.store.setState((state) => ({
          requireFullName: saved.requireFullName,
          requireAvatar: saved.requireAvatar,
          artworkColor: saved.artworkColor,
          organization: {
            ...state.organization,
            name: settings.store.getState().organizationSavedName || settings.store.getState().organization.name || 'Workspace',
            links: [],
            destination: saved.destination,
          },
        }))
    })
  })
</script>

{#if (status === 'loading' || status === 'idle') && !ready}<SettingsContent>
    <Heading>Onboarding</Heading><LoadingRegion ready={false} pending label="Loading onboarding" class="grid gap-12">
      <Section title="Preview"><Skeleton><div class="aspect-21/13 rounded-control bg-inset"></div></Skeleton></Section><Section
        title="General"
      >
        <Rows>
          <Row label="Welcome message"><Skeleton><span class="block h-8 w-64 rounded-lg"></span></Skeleton></Row><Row
            label="Open after onboarding"
            last
          >
            <Skeleton><span class="block h-8 w-44 rounded-lg"></span></Skeleton>
          </Row>
        </Rows>
      </Section><Section title="Appearance">
        <Rows><Row label="Artwork color" last><Skeleton><span class="block h-8 w-16 rounded-lg"></span></Skeleton></Row></Rows>
      </Section>
    </LoadingRegion>
  </SettingsContent>{:else}{#if status !== 'ready' && !ready}<SettingsContent>
      <Heading>Onboarding</Heading><Empty
        title={status === 'denied' ? 'You do not have access' : status === 'error' ? 'Couldn’t load onboarding' : 'Loading…'}
      >
        {#snippet action()}{#if status === 'error'}<Button onclick={() => void controller.load()}>
              Retry
            </Button>{:else}{undefined}{/if}{/snippet}
      </Empty>
    </SettingsContent>{:else}<SettingsContent>
      <Heading>Onboarding</Heading>
      <div class="grid gap-12">
        <Section title="Preview">
          <PreviewBoundary value={preview}><PreviewCanvas></PreviewCanvas></PreviewBoundary>
        </Section><Form.Root model={controller.store.form}>
          <div class="grid gap-12">
            <Section title="General" description="Welcome new members to your workspace.">
              <Rows>
                <Row label="Welcome message" description="A personal greeting from your team.">
                  <TextField name="draft.welcome" aria-label="Welcome message" class="w-64 max-w-full" readonly={!editable}
                  ></TextField><Form.Error name="draft.welcome"></Form.Error>
                </Row><Row label="Open after onboarding" description="Where members land when they finish." last>
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <Select.Control
                      name="draft.destination"
                      aria-label="Open after onboarding"
                      class="w-44"
                      disabled={!editable}
                      options={['CHAT', 'MODELS', 'PROVIDERS'].map((value) => ({
                        value,
                        label: value[0] + value.slice(1).toLowerCase(),
                      }))}
                    ></Select.Control>
                  </DisabledTooltip>
                </Row>
              </Rows>
            </Section><Section title="Appearance" description="Set the mood for your welcome.">
              <Rows>
                <Row label="Artwork color" description="Choose a custom tint for the welcome artwork." last>
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <TextField
                      type="color"
                      value={color ?? '#3850a0'}
                      aria-label="Artwork color"
                      disabled={!editable}
                      oninput={(event) => controller.store.form.change('draft.artworkColor', event.currentTarget.value)}
                    ></TextField>
                  </DisabledTooltip><Form.Error name="draft.artworkColor"></Form.Error>{#if editable}<Button
                      variant="ghost"
                      onclick={() => {
                        controller.store.form.change('draft.artworkColor', null)
                        void controller.store.form.commit(['draft.artworkColor'])
                      }}
                    >
                      Reset
                    </Button>{:else}{/if}
                </Row>
              </Rows>
            </Section><Section title="Profile requirements" description="Choose what new invited members need to share.">
              <Rows>
                <Row label="Require full name" description="Ask new members to add their full name.">
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <Switch name="draft.requireFullName" aria-label="Require full name" disabled={!editable}></Switch>
                  </DisabledTooltip>
                </Row><Row label="Require profile photo" description="Ask new members to upload a profile photo." last>
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <Switch name="draft.requireAvatar" aria-label="Require profile photo" disabled={!editable}></Switch>
                  </DisabledTooltip>
                </Row>
              </Rows>
            </Section>
          </div>
        </Form.Root>
      </div>
    </SettingsContent>{/if}{/if}
