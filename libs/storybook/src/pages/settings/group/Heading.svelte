<script lang="ts">
  import { getSettingsController } from '../../../context'
  const settings = getSettingsController()
  let id = $derived(settings.pages.group.store.getState().draft?.id)
  let name = $derived(settings.store.getState().groups.find((record) => record.id === id)?.name)
</script>

{name ?? 'Group'}
