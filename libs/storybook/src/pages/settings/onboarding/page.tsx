import { LoadingRegion, Skeleton } from '../../../components/loading'
import { useOnboardingController } from '../../onboarding/controller'
import set from 'lodash-es/set.js'
import get from 'lodash-es/get.js'
import { useEffect, useRef, useState } from 'react'
import { useFormStore, useFormValue, Form } from '../../../components/form'
import { OrganizationOnboardingSettingsInput } from '@compallm/contract/validation'
import { Button } from '../../../components/buttons'
import { TextField } from '../../../components/fields'
import { Select } from '../../../components/dropdowns'
import { DisabledTooltip } from '../../../components/feedback'
import { Switch } from '../../../components/selection'
import { usePermission } from '../../../components/permission'
import { OnboardingControllerContext } from '../../../context'
import { OnboardingPage, OnboardingPreviewController } from '../../onboarding'
import { SettingsContent, Heading, Section, Rows, Row, Empty } from '../shared'
import { useSettingsController } from '../controller'

function PreviewDraft() {
  const form = useSettingsController().pages.onboarding!.store.form
  const welcome = useFormValue<string>(form, 'draft.welcome')
  const preview = useOnboardingController()
  useEffect(() => {
    preview.store.setState((state) => ({ organization: { ...state.organization, welcome } }))
  }, [preview, welcome])
  return null
}

function PreviewCanvas() {
  const viewport = useRef<HTMLDivElement>(null)
  const [scale, setScale] = useState(1)
  useEffect(() => {
    const element = viewport.current
    if (!element) return
    const observer = new ResizeObserver(([entry]) => setScale(entry.contentRect.width / 672))
    observer.observe(element)
    return () => observer.disconnect()
  }, [])
  return (
    <div
      className="rounded-4xl bg-linear-to-br from-muted-foreground/50 via-border to-elevated
p-3 shadow-menu ring-1 ring-border"
    >
      <div
        className="relative overflow-hidden rounded-3xl p-6 ring-1 ring-background/70 sm:p-8"
        style={{
          backgroundColor: 'color-mix(in srgb, var(--primary) 9%, var(--background))',
          backgroundImage:
            'radial-gradient(circle, color-mix(in srgb, var(--primary) 18%, transparent) 0.5px, transparent 1px)',
          backgroundSize: '16px 16px',
        }}
      >
        <div ref={viewport} className="relative aspect-21/13 w-full">
          <div
            className="absolute top-0 left-0 w-2xl origin-top-left rounded-3xl shadow-menu
ring-1 ring-border [&_main]:min-h-104"
            style={{ transform: `scale(${scale})` }}
          >
            <OnboardingPage />
          </div>
        </div>
      </div>
    </div>
  )
}

export function OrganizationOnboardingEditor() {
  const settings = useSettingsController()
  const controller = settings.pages.onboarding!
  const status = useFormStore(controller.store, (s) => s.status)
  const saved = useFormStore(controller.store, (s) => s.saved)
  const ready = !!saved
  const color = useFormValue<string | null>(controller.store.form, 'draft.artworkColor')
  const permission = usePermission({ actions: ['organization.onboarding.update'] })
  const editable = permission.allowed
  const [preview] = useState(() => new OnboardingPreviewController())
  useEffect(() => {
    void controller.load()
    return controller.cancel
  }, [controller])
  useEffect(() => {
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
        const candidate = structuredClone(controller.store.getState().saved ?? values.draft!)
        for (const name of names) set(candidate!, name.slice('draft.'.length), get(values, name))
        const parsed = OrganizationOnboardingSettingsInput.safeParse(candidate)
        return parsed.success ? { values } : { issues: parsed.error.issues.map((i) => ({ ...i, path: ['draft', ...i.path] })) }
      },
      error: (error, retry) => settings.apiError(error, retry),
      submit: async (_values, patch, signal) => ({ draft: await controller.update(patch.draft ?? {}, signal) }),
    })
    return () => form.stop()
  }, [controller, settings, ready])
  useEffect(() => {
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
  }, [saved, preview, settings])
  if ((status === 'loading' || status === 'idle') && !ready)
    return (
      <SettingsContent>
        <Heading>Onboarding</Heading>
        <LoadingRegion ready={false} pending label="Loading onboarding" className="grid gap-12">
          <Section title="Preview">
            <Skeleton>
              <div className="aspect-21/13 rounded-control bg-inset" />
            </Skeleton>
          </Section>
          <Section title="General">
            <Rows>
              <Row label="Welcome message">
                <Skeleton>
                  <span className="block h-8 w-64 rounded-lg" />
                </Skeleton>
              </Row>
              <Row label="Open after onboarding" last>
                <Skeleton>
                  <span className="block h-8 w-44 rounded-lg" />
                </Skeleton>
              </Row>
            </Rows>
          </Section>
          <Section title="Appearance">
            <Rows>
              <Row label="Artwork color" last>
                <Skeleton>
                  <span className="block h-8 w-16 rounded-lg" />
                </Skeleton>
              </Row>
            </Rows>
          </Section>
        </LoadingRegion>
      </SettingsContent>
    )
  if (status !== 'ready' && !ready)
    return (
      <SettingsContent>
        <Heading>Onboarding</Heading>
        <Empty
          title={status === 'denied' ? 'You do not have access' : status === 'error' ? 'Couldn’t load onboarding' : 'Loading…'}
          action={status === 'error' ? <Button onClick={() => void controller.load()}>Retry</Button> : undefined}
        >
          {' '}
        </Empty>
      </SettingsContent>
    )
  return (
    <SettingsContent>
      <Heading>Onboarding</Heading>
      <div className="grid gap-12">
        <Section title="Preview">
          <OnboardingControllerContext value={preview}>
            <PreviewDraft />
            <PreviewCanvas />
          </OnboardingControllerContext>
        </Section>
        <Form.Root model={controller.store.form}>
          <div className="grid gap-12">
            <Section title="General" description="Welcome new members to your workspace.">
              <Rows>
                <Row label="Welcome message" description="A personal greeting from your team.">
                  <TextField
                    name="draft.welcome"
                    aria-label="Welcome message"
                    className="w-64 max-w-full"
                    readOnly={!editable}
                  />
                  <Form.Error name="draft.welcome" />
                </Row>
                <Row label="Open after onboarding" description="Where members land when they finish." last>
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <Select.Control
                      name="draft.destination"
                      aria-label="Open after onboarding"
                      className="w-44"
                      disabled={!editable}
                      options={['CHAT', 'MODELS', 'PROVIDERS'].map((value) => ({
                        value,
                        label: value[0] + value.slice(1).toLowerCase(),
                      }))}
                    />
                  </DisabledTooltip>
                </Row>
              </Rows>
            </Section>
            <Section title="Appearance" description="Set the mood for your welcome.">
              <Rows>
                <Row label="Artwork color" description="Choose a custom tint for the welcome artwork." last>
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <TextField
                      type="color"
                      value={color ?? '#3850a0'}
                      aria-label="Artwork color"
                      disabled={!editable}
                      onChange={(event) => controller.store.form.change('draft.artworkColor', event.target.value)}
                    />
                  </DisabledTooltip>
                  <Form.Error name="draft.artworkColor" />
                  {editable ? (
                    <Button
                      variant="ghost"
                      onClick={() => {
                        controller.store.form.change('draft.artworkColor', null)
                        void controller.store.form.commit(['draft.artworkColor'])
                      }}
                    >
                      Reset
                    </Button>
                  ) : null}
                </Row>
              </Rows>
            </Section>
            <Section title="Profile requirements" description="Choose what new invited members need to share.">
              <Rows>
                <Row label="Require full name" description="Ask new members to add their full name.">
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <Switch name="draft.requireFullName" aria-label="Require full name" disabled={!editable} />
                  </DisabledTooltip>
                </Row>
                <Row label="Require profile photo" description="Ask new members to upload a profile photo." last>
                  <DisabledTooltip disabled={!editable} reason="You do not have permission to edit onboarding.">
                    <Switch name="draft.requireAvatar" aria-label="Require profile photo" disabled={!editable} />
                  </DisabledTooltip>
                </Row>
              </Rows>
            </Section>
          </div>
        </Form.Root>
      </div>
    </SettingsContent>
  )
}
