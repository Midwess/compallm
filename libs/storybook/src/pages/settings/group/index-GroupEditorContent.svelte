<script lang="ts">
  import { PermissionRequest, usePermission } from '../../../components/permission'

  import { FormError } from '../../../components/form'
  import { DisabledTooltip } from '../../../components/feedback'

  import { Button } from '../../../components/buttons'

  import { TextField } from '../../../components/fields'

  import { SettingsContent, Heading, Section, Rows, Row, Empty, SettingsDetailLoading } from '../shared'
  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'
  import { GroupMembers, GroupRoles } from './index'
  import { GroupHeading } from './index'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let loading = $derived(readStore(__settings.pages.group.store, (s) => s.loading))
  let draft = $derived(readStore(__settings.pages.group.store, (s) => (s.draft ? { id: s.draft.id } : null), ['draft.id']))
  let readonly = $derived(readStore(__settings.store, (s) => s.readonly))
  let busy = $derived(readStore(__settings.pages.group.store, (s) => s.busy))
  let confirmedTitle = $derived(
    readStore(__settings.store, (s) => s.groups.find((group) => group.id === s.location.id)?.name ?? 'Group'),
  )
  const metadataPermission = usePermission(() => ({
    actions: ['group.metadata.update'],
    resource: { type: 'Group', id: draft?.id },
    enabled: !!draft?.id,
  }))
</script>

{#if loading && !draft}<SettingsDetailLoading title={confirmedTitle} sections={['Group details', 'Roles', 'Members', 'Danger']}
  ></SettingsDetailLoading>{:else}{#if !draft}<SettingsContent>
      <Empty title="Group unavailable">Choose a group from the sidebar.</Empty>
    </SettingsContent>{:else}<SettingsContent class="flex flex-col pt-14 pb-6">
      <Heading class="mb-8 shrink-0"><GroupHeading></GroupHeading></Heading>
      <div class="flex min-w-0 flex-col gap-6">
        <Section title="Group details" class="shrink-0">
          <Rows class="**:[[role=separator]]:bg-border">
            <Row>
              {#snippet label()}<label for="group-name">Name</label>{/snippet}<DisabledTooltip
                disabled={readonly || busy || !metadataPermission.allowed}
                reason={!metadataPermission.allowed
                  ? 'You do not have permission to edit this group.'
                  : readonly
                    ? 'This group is read-only.'
                    : 'Wait for the current update to finish.'}
              >
                <TextField
                  id="group-name"
                  class="w-xl max-w-full border-border max-lg:w-sm"
                  name="draft.name"
                  placeholder="Group name…"
                  disabled={readonly || busy || !metadataPermission.allowed}
                ></TextField>
              </DisabledTooltip><FormError name="draft.name"></FormError>
            </Row><Row last>
              {#snippet label()}<label for="group-description">Description</label>{/snippet}<DisabledTooltip
                disabled={readonly || busy || !metadataPermission.allowed}
                reason={!metadataPermission.allowed
                  ? 'You do not have permission to edit this group.'
                  : readonly
                    ? 'This group is read-only.'
                    : 'Wait for the current update to finish.'}
              >
                <TextField
                  id="group-description"
                  class="w-xl max-w-full border-border max-lg:w-sm"
                  name="draft.description"
                  placeholder="Describe this group…"
                  disabled={readonly || busy || !metadataPermission.allowed}
                ></TextField>
              </DisabledTooltip><FormError name="draft.description"></FormError>
            </Row>
          </Rows>
        </Section><PermissionRequest actions={['role.read']}>
          {#snippet children(allowed)}{#if allowed}<GroupRoles></GroupRoles>{:else}{/if}{/snippet}
        </PermissionRequest><PermissionRequest actions={['member.read']}>
          {#snippet children(allowed)}{#if allowed}<GroupMembers></GroupMembers>{:else}{/if}{/snippet}
        </PermissionRequest>{#if !readonly && draft.id}<PermissionRequest
            actions={['group.delete']}
            resource={{ type: 'Group', id: draft.id }}
          >
            {#snippet children(allowed)}{#if allowed}<Section title="Danger" class="shrink-0">
                  <Rows>
                    <Row label="Delete group" last>
                      <Button
                        variant="ghost"
                        class="text-destructive"
                        disabled={busy}
                        onclick={() => c.pages.dialogs.openDialog('delete-group', draft.id!)}
                      >
                        Delete group
                      </Button>
                    </Row>
                  </Rows>
                </Section>{:else}{/if}{/snippet}
          </PermissionRequest>{:else}{/if}
      </div>
    </SettingsContent>{/if}{/if}
