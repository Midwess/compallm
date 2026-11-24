<script lang="ts">
  import { usePermission } from '../../../components/permission'
  import { FormError } from '../../../components/form'

  import { TextField } from '../../../components/fields'

  import { DisabledTooltip } from '../../../components/feedback'
  import { Section, Rows, Row } from '../shared'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  import { ownerExplanation } from './index'

  const __settings = getSettingsController()
  let id = $derived(readStore(__settings.pages.role.store, (state) => state.draft?.id, ['draft.id']))
  let builtin = $derived(readStore(__settings.pages.role.store, (state) => state.draft?.builtin, ['draft.builtin']))
  const metadataPermission = usePermission(() => ({
    actions: ['role.metadata.update'],
    resource: { type: 'Role', id },
    enabled: !!id,
  }))
</script>

<Section title="Role details" class="shrink-0">
  <Rows class="**:[[role=separator]]:bg-border">
    <Row>
      {#snippet label()}<label for="role-name">Name</label>{/snippet}<DisabledTooltip
        disabled={builtin || !metadataPermission.allowed}
        reason={builtin ? ownerExplanation : 'You do not have permission to edit this role.'}
      >
        <TextField
          id="role-name"
          class="w-xl max-w-full border-border max-lg:w-sm"
          placeholder="Role name…"
          name="draft.name"
          disabled={builtin || !metadataPermission.allowed}
        ></TextField><FormError name="draft.name"></FormError>
      </DisabledTooltip>
    </Row><Row last>
      {#snippet label()}<label for="role-description">Description</label>{/snippet}<DisabledTooltip
        disabled={builtin || !metadataPermission.allowed}
        reason={builtin ? ownerExplanation : 'You do not have permission to edit this role.'}
      >
        <TextField
          id="role-description"
          class="w-xl max-w-full border-border max-lg:w-sm"
          placeholder="Describe this role…"
          name="draft.description"
          disabled={builtin || !metadataPermission.allowed}
        ></TextField><FormError name="draft.description"></FormError>
      </DisabledTooltip>
    </Row>
  </Rows>
</Section>
