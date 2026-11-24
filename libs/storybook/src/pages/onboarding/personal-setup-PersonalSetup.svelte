<script lang="ts">
  import {
    UserProfileUpdateInput,
    ProfileUpdateInput,
    ImageUploadInput,
    AnalyticUserProfilePutInput,
    OrganizationProfileUpdateInput,
    PreferenceAppearanceMode,
  } from '@compallm/contract/validation'
  import { Button } from '../../components/buttons'
  import { Avatar } from '../../components/content'
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import { TextField } from '../../components/fields'
  import { Select } from '../../components/dropdowns'
  import { Form, managedForm } from '../../components/form'
  import { ChoiceCard } from '../../components/selection'

  import { OnboardingLayout } from './layout'
  import type { OnboardingStep } from './types'
  import { untrack } from 'svelte'
  import { readStore, readForm, systemLight as readSystemLight } from '../page-state.svelte'
  import { getOnboardingController } from '../../context'
  import { workOptions } from './personal-setup'
  import ContentValue from '../../components/Content.svelte'

  let {
    step,
  }: {
    step: OnboardingStep
  } = $props()
  const __onboarding = getOnboardingController()
  const controller = untrack(() => getOnboardingController())
  let saving = $derived(readStore(__onboarding.store, (state) => state.pending))
  let initial = $derived(controller.store.getState())
  const id = $props.id()
  let systemLight = $derived(readSystemLight())
  let profile = $derived(step === 'welcome' || step === 'profile')
  let work = $derived(step === 'work')
  let workspace = $derived(step === 'workspace')
  let fileInput = $state<HTMLInputElement | null>(null)
  let localPhoto = $state<string | null>(null)
  let preview = $state(untrack(() => initial.profile.avatarUrl))
  $effect(() => {
    void []
    return untrack(() => () => {
      if (localPhoto) URL.revokeObjectURL(localPhoto)
    })
  })
  let knownWork = $derived(workOptions.some((label) => label.toLowerCase() === initial.workType))
  const form = untrack(() =>
    managedForm(
      {
        photo: null as File | null,
        username: initial.profile.username,
        displayName: initial.profile.displayName,
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
            if (initial.requireFullName) {
              const requiredName = ProfileUpdateInput.shape.name.unwrap().safeParse(values.displayName)
              if (!requiredName.success)
                return { issues: requiredName.error.issues.map((issue) => ({ ...issue, path: ['displayName'] })) }
            }
            if (values.photo || (initial.requireAvatar && !initial.profile.avatarUrl)) {
              const image = ImageUploadInput.safeParse({
                size: values.photo?.size ?? 0,
                mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[
                  values.photo?.type ?? 'image/jpeg'
                ],
              })
              if (!image.success) return { issues: image.error.issues.map((issue) => ({ ...issue, path: ['photo'] })) }
            }
            const user = UserProfileUpdateInput.safeParse({
              username: values.username,
              ...(initial.requireFullName ? { displayName: values.displayName } : {}),
            })
            return user.success ? { values: { ...values, username: user.data.username! } } : { issues: user.error.issues }
          }
          if (work) {
            const value = values.workType === 'other' ? values.otherWorkType.trim() || 'other' : values.workType
            const answer = AnalyticUserProfilePutInput.safeParse({ key: 'work', value })
            if (!answer.success) return { issues: answer.error.issues.map((issue) => ({ ...issue, path: ['otherWorkType'] })) }
          }
          if (!workspace || values.workspaceName === initial.organization.name) return { values }
          const org = OrganizationProfileUpdateInput.safeParse({ name: values.workspaceName })
          return org.success
            ? { values: { ...values, workspaceName: org.data.name! } }
            : { issues: org.error.issues.map((issue) => ({ ...issue, path: ['workspaceName'] })) }
        },
        submit: async (values, _patch, signal) => {
          let accepted = false
          if (profile)
            accepted = Boolean(
              await controller.saveProfile(
                { username: values.username, ...(initial.requireFullName ? { displayName: values.displayName } : {}) },
                values.photo,
                signal,
              ),
            )
          if (workspace) accepted = await controller.saveWorkspaceName(values.workspaceName, signal)
          if (work)
            accepted = await controller.saveWorkType(
              values.workType === 'other' ? values.otherWorkType.trim() || 'other' : values.workType,
              signal,
            )
          if (step === 'preferences') accepted = await controller.saveAppearance(values.appearance, signal)
          if (!accepted || signal.aborted) return false
          await controller.next()
          return values
        },
      },
    ),
  )
  let pending = $derived((() => form.pending)())
  let appearance = $derived(readForm<string>(form, 'appearance'))
  let workType = $derived(readForm<string>(form, 'workType'))
  let username = $derived(readForm<string>(form, 'username'))
  let light = $derived(step === 'preferences' && (appearance === 'LIGHT' || (appearance === 'SYSTEM' && systemLight)))
</script>

<Form.Root
  model={form}
  aria-label="Personal setup"
  data-theme={step === 'preferences' ? (light ? 'basic-white' : 'basic-dark') : undefined}
>
  <OnboardingLayout
    title={profile
      ? 'Welcome to Compallm'
      : workspace
        ? 'Name your workspace'
        : work
          ? 'What kind of work do you do?'
          : 'Make yourself at home'}
    description={profile
      ? 'Make yourself at home with a name and photo.\nYou can always change them later.'
      : workspace
        ? 'Give your ideas a space to call home.\nKeep the name, or choose one that feels right.'
        : work
          ? 'Tell us a little about the work you do.\nShare what fits, or keep it to yourself.'
          : 'Choose a look that feels comfortable.\nWe’ll keep things just the way you like.'}
    progress={profile ? 0.28 : workspace ? 0.4 : work ? 0.58 : 0.75}
  >
    {#snippet action()}<Button
        type="submit"
        variant="primary"
        size="authentication"
        loading={pending || saving}
        class="h-12 w-full text-base"
      >
        Continue
      </Button>{/snippet}{#if profile}<div class="grid gap-2">
        <div class="flex items-end gap-3">
          <div class="grid min-w-0 flex-1 gap-2">
            <label for={`${id}-username`} class="text-sm text-muted-foreground">Username </label><TextField
              id={`${id}-username`}
              name="username"
              class="h-10 text-base"
            ></TextField><Form.Error name="username"></Form.Error>
          </div>
          <Button
            variant="ghost"
            data-form-field="photo"
            data-ui="avatar-input"
            aria-label="Upload profile photo"
            class="group relative size-9 mb-0.5 shrink-0 overflow-hidden rounded-full p-0"
            onclick={() => fileInput?.click()}
          >
            <Avatar name={username} src={preview ?? undefined} size={36}></Avatar><span
              aria-hidden="true"
              class="absolute inset-0 grid place-items-center rounded-full
bg-overlay/60 text-on-overlay opacity-0 group-hover:opacity-100 group-focus-visible:opacity-100"
            >
              <PlusIcon size={16}></PlusIcon>
            </span>
          </Button><input
            bind:this={fileInput}
            name="photo"
            type="file"
            required={initial.requireAvatar && !initial.profile.avatarUrl}
            accept="image/jpeg,image/png,image/webp"
            aria-label="Profile photo"
            class="sr-only"
            tabindex={-1}
            oninput={(event) => {
              const file = event.currentTarget.files?.[0]
              if (!file) return
              if (localPhoto) URL.revokeObjectURL(localPhoto)
              localPhoto = URL.createObjectURL(file)
              preview = localPhoto
              form.change('photo', file)
            }}
          /><Form.Error name="photo"></Form.Error>
        </div>
        {#if initial.requireFullName}<label class="grid gap-2 text-sm text-muted-foreground">
            Full name <TextField name="displayName" class="h-10 text-base" required></TextField><Form.Error name="displayName"
            ></Form.Error>
          </label>{:else}{/if}
      </div>{:else}{#if workspace}<div class="grid gap-2">
          <label for={`${id}-workspace`} class="text-sm text-muted-foreground">Workspace name </label><TextField
            id={`${id}-workspace`}
            name="workspaceName"
            class="h-10 text-base"
          ></TextField><Form.Error name="workspaceName"></Form.Error>
        </div>{:else}{#if work}<div class="grid gap-2">
            <label for={`${id}-work`} class="text-sm text-muted-foreground">Your work </label><Select.Root
              name="workType"
              items={workOptions.map((label) => ({ label, value: label.toLowerCase() }))}
            >
              <Select.Trigger id={`${id}-work`} aria-label="Your work" class="h-12 px-3 text-base">
                <Select.Value></Select.Value>
              </Select.Trigger><Select.Content aria-label="Your work">
                {#each workOptions as label (label)}<Select.Item value={label.toLowerCase()} {label}>
                    <ContentValue value={label} />
                  </Select.Item>{/each}
              </Select.Content>
            </Select.Root>{#if workType === 'other'}<label class="grid gap-2 text-sm text-muted-foreground">
                Let us know <TextField name="otherWorkType" placeholder="Tell us about your work" class="h-10 text-base"
                ></TextField><Form.Error name="otherWorkType"></Form.Error>
              </label>{:else}{/if}
          </div>{:else}<fieldset class="grid gap-2">
            <legend class="sr-only">Appearance</legend>{#each PreferenceAppearanceMode.options as value (value)}<ChoiceCard
                name="appearance"
                {value}
                class="min-h-10 w-full gap-2 px-4 [&_strong]:text-base"
              >
                {#if value === 'SYSTEM'}Use system setting{:else}{#if value === 'LIGHT'}Light{:else}Dark{/if}{/if}
              </ChoiceCard>{/each}
          </fieldset>{/if}{/if}{/if}
  </OnboardingLayout>
</Form.Root>
