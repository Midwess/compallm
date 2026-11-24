<script lang="ts">
  import { getSettingsController } from '../../../context'
  const settings = getSettingsController()
  let id = $derived(settings.pages.role.store.getState().draft?.id)
  let name = $derived(settings.store.getState().roles.find((record) => record.id === id)?.name)
</script>

{name ?? 'New role'}
