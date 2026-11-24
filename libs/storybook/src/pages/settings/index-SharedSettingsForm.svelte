<script lang="ts">
  import { configureSettingsForm } from './form'
  import { Form } from '../../components/form'

  import { UserPreferencePage } from './preferences/user'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { ProtectedSettingsScreen } from './index'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let page = $derived(readStore(__settings.store, (s) => s.location.page))
  let form = $derived(c.store.form)
  let ready = $derived(
    readStore(__settings.store, (s) =>
      page === 'profile' ? s.profileLoaded !== false : page === 'organization' ? s.organizationProfileLoaded !== false : true,
    ),
  )
  $effect(() => {
    void [c, form, page, ready]
    return untrack(() => {
      if (!ready) return
      return configureSettingsForm(c)
    })
  })
</script>

{#if ['general', 'messaging', 'code'].includes(page)}<UserPreferencePage>
    <ProtectedSettingsScreen></ProtectedSettingsScreen>
  </UserPreferencePage>{:else}{#if ['role', 'new-role', 'group', 'new-group', 'members', 'organization-onboarding', 'security'].includes(page)}<ProtectedSettingsScreen
    ></ProtectedSettingsScreen>{:else}<Form.Root model={form}>
      <ProtectedSettingsScreen></ProtectedSettingsScreen>
    </Form.Root>{/if}{/if}
