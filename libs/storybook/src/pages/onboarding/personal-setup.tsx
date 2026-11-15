import { UserProfileUpdateInput, OrganizationProfileUpdateInput, PreferenceAppearanceMode } from '@compallm/contract/validation'
import { useId, useSyncExternalStore, useState, useEffect, useRef } from 'react'
import { Button } from '../../components/buttons'
import { Avatar } from '../../components/content'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { TextField } from '../../components/fields'
import { Select } from '../../components/dropdowns'
import { Form, useManagedForm, useFormValue } from '../../components/form'
import { ChoiceCard } from '../../components/selection'
import { useSystemLight } from '../../components/code-appearance'
import { useOnboardingController } from './controller'
import { OnboardingLayout } from './layout'
import type { OnboardingStep } from './types'

const workOptions = ['Engineering', 'Product', 'Design', 'Research', 'Writing & marketing', 'Operations', 'Studying', 'Other']

export function PersonalSetup({ step }: { step: OnboardingStep }) {
  const controller = useOnboardingController()
  const initial = controller.store.getState()
  const id = useId()
  const systemLight = useSystemLight()
  const profile = step === 'welcome' || step === 'profile'
  const work = step === 'work'
  const workspace = step === 'workspace'
  const fileInput = useRef<HTMLInputElement>(null)
  const localPhoto = useRef<string | null>(null)
  const [preview, setPreview] = useState(initial.profile.avatarUrl)
  useEffect(
    () => () => {
      if (localPhoto.current) URL.revokeObjectURL(localPhoto.current)
    },
    [],
  )
  const knownWork = workOptions.some((label) => label.toLowerCase() === initial.workType)
  const form = useManagedForm(
    {
      photo: null as File | null,
      username: initial.profile.username,
      workspaceName: initial.organization.name,
      workType: initial.workType ? (knownWork ? initial.workType : 'other') : 'engineering',
      otherWorkType: knownWork ? '' : initial.workType,
      appearance: initial.appearance,
    },
    {
      mode: 'button',
      command: true,
      parse: (values) => {
        if (profile) {
          const user = UserProfileUpdateInput.safeParse({ username: values.username })
          return user.success ? { values: { ...values, username: user.data.username! } } : { issues: user.error.issues }
        }
        if (!workspace) return { values }
        const org = OrganizationProfileUpdateInput.safeParse({ name: values.workspaceName })
        return org.success
          ? { values: { ...values, workspaceName: org.data.name! } }
          : { issues: org.error.issues.map((issue) => ({ ...issue, path: ['workspaceName'] })) }
      },
      submit: async (values, _patch, signal) => {
        let accepted = false
        if (profile) accepted = Boolean(await controller.saveProfile({ username: values.username }, values.photo, signal))
        if (workspace) accepted = await controller.saveWorkspaceName(values.workspaceName, signal)
        if (work)
          accepted = await controller.saveWorkType(
            values.workType === 'other' ? values.otherWorkType.trim() || 'other' : values.workType,
            signal,
          )
        if (step === 'preferences') accepted = await controller.saveAppearance(values.appearance, signal)
        if (!accepted || signal.aborted) return false
        controller.next()
        return values
      },
    },
  )
  const pending = useSyncExternalStore(
    form.subscribe,
    () => form.pending,
    () => false,
  )
  const appearance = useFormValue<string>(form, 'appearance')
  const workType = useFormValue<string>(form, 'workType')
  const username = useFormValue<string>(form, 'username')
  const light = step === 'preferences' && (appearance === 'LIGHT' || (appearance === 'SYSTEM' && systemLight))
  return (
    <Form.Root
      model={form}
      aria-label="Personal setup"
      data-theme={step === 'preferences' ? (light ? 'basic-white' : 'basic-dark') : undefined}
    >
      <OnboardingLayout
        title={
          profile
            ? 'Welcome to Compallm'
            : workspace
              ? 'Name your workspace'
              : work
                ? 'What kind of work do you do?'
                : 'Make yourself at home'
        }
        description={
          profile
            ? 'Make yourself at home with a name and photo.\nYou can always change them later.'
            : workspace
              ? 'Give your ideas a space to call home.\nKeep the name, or choose one that feels right.'
              : work
                ? 'Tell us a little about the work you do.\nShare what fits, or keep it to yourself.'
                : 'Choose a look that feels comfortable.\nWe’ll keep things just the way you like.'
        }
        progress={profile ? 0.28 : workspace ? 0.4 : work ? 0.58 : 0.75}
        action={
          <Button type="submit" variant="primary" size="authentication" loading={pending} className="h-12 w-full text-base">
            Continue
          </Button>
        }
      >
        {profile ? (
          <div className="grid gap-3">
            <div className="flex items-end gap-3">
              <div className="grid min-w-0 flex-1 gap-2">
                <label htmlFor={`${id}-username`} className="text-sm text-muted-foreground">
                  Username
                </label>
                <TextField id={`${id}-username`} name="username" className="h-10 text-base" />
                <Form.Error name="username" />
              </div>
              <Button
                variant="ghost"
                aria-label="Upload profile photo"
                className="group relative size-9 mb-0.5 shrink-0 overflow-hidden rounded-full p-0"
                onClick={() => fileInput.current?.click()}
              >
                <Avatar name={username} src={preview ?? undefined} size={36} />
                <span
                  aria-hidden="true"
                  className="absolute inset-0 grid place-items-center rounded-full
bg-overlay/60 text-on-overlay opacity-0 group-hover:opacity-100 group-focus-visible:opacity-100"
                >
                  <PlusIcon size={16} />
                </span>
              </Button>
              <input
                ref={fileInput}
                type="file"
                accept="image/jpeg,image/png,image/webp"
                aria-label="Profile photo"
                className="sr-only"
                tabIndex={-1}
                onChange={(event) => {
                  const file = event.target.files?.[0]
                  if (!file) return
                  if (localPhoto.current) URL.revokeObjectURL(localPhoto.current)
                  localPhoto.current = URL.createObjectURL(file)
                  setPreview(localPhoto.current)
                  form.change('photo', file)
                  event.target.value = ''
                }}
              />
            </div>
          </div>
        ) : workspace ? (
          <div className="grid gap-2">
            <label htmlFor={`${id}-workspace`} className="text-sm text-muted-foreground">
              Workspace name
            </label>
            <TextField id={`${id}-workspace`} name="workspaceName" className="h-10 text-base" />
            <Form.Error name="workspaceName" />
          </div>
        ) : work ? (
          <div className="grid gap-2">
            <label htmlFor={`${id}-work`} className="text-sm text-muted-foreground">
              Your work
            </label>
            <Select.Root name="workType" items={workOptions.map((label) => ({ label, value: label.toLowerCase() }))}>
              <Select.Trigger id={`${id}-work`} aria-label="Your work" className="h-12 px-3 text-base">
                <Select.Value />
              </Select.Trigger>
              <Select.Content aria-label="Your work">
                {workOptions.map((label) => (
                  <Select.Item key={label} value={label.toLowerCase()} label={label}>
                    {label}
                  </Select.Item>
                ))}
              </Select.Content>
            </Select.Root>
            {workType === 'other' ? (
              <label className="grid gap-2 text-sm text-muted-foreground">
                Let us know
                <TextField name="otherWorkType" placeholder="Tell us about your work" className="h-10 text-base" />
              </label>
            ) : null}
          </div>
        ) : (
          <fieldset className="grid gap-3">
            <legend className="sr-only">Appearance</legend>
            {PreferenceAppearanceMode.options.map((value) => (
              <ChoiceCard
                key={value}
                name="appearance"
                value={value}
                className="min-h-13 w-full gap-2 px-4 [&_strong]:text-base"
              >
                {value === 'SYSTEM' ? 'Use system setting' : value === 'LIGHT' ? 'Light' : 'Dark'}
              </ChoiceCard>
            ))}
          </fieldset>
        )}
      </OnboardingLayout>
    </Form.Root>
  )
}
