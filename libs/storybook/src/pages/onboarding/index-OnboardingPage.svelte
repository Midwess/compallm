<script lang="ts">
  import { Button } from '../../components/buttons'
  import { Checkbox } from '../../components/selection'
  import { SectionSurface } from '../../components/surfaces'
  import CelebrationMotion from '../../components/CelebrationMotion.svelte'

  import { OnboardingLayout } from './layout'
  import { PersonalSetup } from './personal-setup'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getOnboardingController } from '../../context'
  import { Detail, OnboardingToast } from './index'

  const __onboarding = getOnboardingController()
  const controller = untrack(() => getOnboardingController())
  let step = $derived(readStore(__onboarding.store, (s) => s.step))
  let profile = $derived(readStore(__onboarding.store, (s) => s.profile))
  let org = $derived(readStore(__onboarding.store, (s) => s.organization))
  let invited = $derived(readStore(__onboarding.store, (s) => s.invited))
  let existingUser = $derived(readStore(__onboarding.store, (s) => s.existingUser))
  let appearance = $derived(readStore(__onboarding.store, (s) => s.appearance))
  let error = $derived(readStore(__onboarding.store, (s) => s.error))
  let completed = $derived(readStore(__onboarding.store, (s) => s.completed))
  let pending = $derived(readStore(__onboarding.store, (s) => s.pending))
  let ready = $derived(step === 'ready' || completed)
  let organization = $derived(step === 'organization' || (ready && invited))
  let introduction = $derived(step === 'introduction')
  let explore = $derived(step === 'explore')
  let title = $derived(
    ready
      ? 'You’re all set!'
      : organization
        ? `Welcome to ${org.name}`
        : introduction
          ? 'Make yourself at home'
          : explore
            ? 'Make room for your ideas'
            : 'Welcome to Compallm',
  )
  let description = $derived(
    ready
      ? `Welcome to ${org.name}`
      : organization
        ? `A new space, and a team to explore it with.\nMake yourself at home in ${org.name}.`
        : introduction
          ? org.welcome || 'Your team’s workspace is ready.\nHere are a few useful places to start.'
          : explore
            ? 'Find a little inspiration for what comes next.'
            : 'A workspace for your ideas, conversations, and AI models.\nLet’s make it yours.',
  )
</script>

<div class="relative" data-theme={appearance === 'SYSTEM' ? undefined : appearance === 'LIGHT' ? 'basic-white' : 'basic-dark'}>
  {#if ['welcome', 'profile', 'workspace', 'work', 'preferences'].includes(step) && !completed}<PersonalSetup {step}
    ></PersonalSetup>{:else}<OnboardingLayout
      {title}
      {description}
      progress={ready ? 1 : organization ? (existingUser ? 0.65 : 0.8) : introduction ? 0.9 : explore ? 0.58 : 0.28}
    >
      {#snippet action()}<Button
          loading={pending}
          variant="primary"
          size="authentication"
          class="h-12 w-full text-base"
          onclick={ready ? controller.finish : controller.next}
        >
          {#if ready}{#if invited}{`Open ${org.name}`}{:else}Open my workspace{/if}{:else}Continue{/if}
        </Button>{/snippet}{#if ready}<div class="relative h-48 w-full" aria-label="Workspace opening celebration">
          <CelebrationMotion active={!completed}></CelebrationMotion>
        </div>{:else}{#if explore}<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {#each ['Chat', 'Models', 'Providers', 'Plugins'] as name (name)}<SectionSurface
                class="flex min-h-13 items-center gap-3 px-4 py-3"
              >
                <label class="flex w-full cursor-pointer items-center gap-3 text-base font-medium">
                  <Checkbox aria-label={name} defaultChecked></Checkbox>{name}
                </label>
              </SectionSurface>{/each}
          </div>{:else}{#if introduction}<div class="grid gap-3">
              {#each org.links as link (link.href)}<SectionSurface class="p-0">
                  <a
                    class="flex h-15 w-full items-center justify-start rounded-lg px-4 text-base hover:bg-hover focus-visible:outline-2 focus-visible:outline-ring"
                    href={link.href}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {link.label}↗
                  </a>
                </SectionSurface>{/each}{#if !org.links.length}<Detail label="Your organization" value={org.name}
                ></Detail>{:else}{/if}
            </div>{:else}<div class="grid gap-3">
              <Detail
                label={ready ? (invited ? 'Your organization' : org.name) : organization ? 'Signed in as' : 'Your account'}
                value={ready ? (invited ? org.name : 'Your personal organization') : profile.email}
              ></Detail><Detail
                label={ready ? 'Your role' : organization ? 'Organization' : 'Your workspace'}
                value={ready ? (invited ? 'Member' : 'Owner') : org.name}
              ></Detail>
            </div>{/if}{/if}{/if}
    </OnboardingLayout>{/if}{#if error}<OnboardingToast message={error} onDismiss={controller.dismissError}
    ></OnboardingToast>{:else}{/if}
</div>
