<script lang="ts">
  import { PermissionRequest, usePermission } from '../../../components/permission'

  import { useTransition } from '../../../components/motion'
  import { Button } from '../../../components/buttons'

  import { SettingsSectionTitle } from '../../../components/settings'
  import { PolicyTable } from './policy-table'
  import { DisabledTooltip } from '../../../components/feedback'
  import { SettingsContent, Heading, Section, Rows, Row, Empty, SettingsDetailLoading } from '../shared'
  import { untrack } from 'svelte'

  import { readStore, toStyle } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'
  import { EditingMode, CedarEditor, RoleDetails } from './index'
  import { ownerExplanation, policyPanelVariants, RoleHeading } from './index'

  import MotionSurface from '../../../components/MotionSurface.svelte'

  const __settings = getSettingsController()
  let timing = $derived(useTransition('panel'))
  const shared = untrack(() => getSettingsController())
  let draft = $derived(
    readStore(
      __settings.pages.role.store,
      (s) =>
        s.draft
          ? {
              id: s.draft.id,
              builtin: s.draft.builtin,
              mode: s.draft.mode,
              parsing: s.draft.parsing,
              diagnostics: s.draft.diagnostics,
            }
          : null,
      ['draft.id', 'draft.builtin', 'draft.mode', 'draft.parsing', 'draft.diagnostics'],
    ),
  )
  let readonly = $derived(readStore(__settings.store, (s) => s.readonly))
  let loading = $derived(readStore(__settings.pages.role.store, (s) => s.loading))
  let busy = $derived(readStore(__settings.pages.role.store, (s) => s.busy))
  let confirmedTitle = $derived(
    readStore(__settings.store, (s) => s.roles.find((role) => role.id === s.location.id)?.name ?? 'Role'),
  )
  const policyPermission = usePermission(() => ({
    actions: ['role.policy.update'],
    resource: { type: 'Role', id: draft?.id },
    enabled: !!draft?.id,
  }))
</script>

{#if loading && !draft}<SettingsDetailLoading title={confirmedTitle} sections={['Role details', 'Permissions', 'Danger']}
  ></SettingsDetailLoading>{:else}{#if !draft}<SettingsContent>
      <Empty title="Role unavailable">Choose a role from the sidebar.</Empty>
    </SettingsContent>{:else}{@const locked = readonly || draft.builtin}{@const invalid =
      draft.parsing || draft.diagnostics.length > 0}<SettingsContent class="flex flex-col pt-14 pb-6">
      <Heading class="mb-8 shrink-0"><RoleHeading></RoleHeading></Heading>
      <div class="flex min-w-0 flex-1 flex-col gap-6">
        <RoleDetails></RoleDetails><PermissionRequest actions={['role.policy.read']} resource={{ type: 'Role', id: draft.id }}>
          {#snippet children(allowed)}{#if allowed}<section
                aria-labelledby="role-permissions-title"
                class="flex min-h-0 min-w-0 flex-1 flex-col gap-2"
              >
                <div class="grid shrink-0 justify-items-start gap-2 px-4 pt-1">
                  <SettingsSectionTitle id="role-permissions-title" class="leading-6">
                    Permissions
                  </SettingsSectionTitle><EditingMode></EditingMode>
                </div>
                <div class="relative grid min-w-0 w-full">
                  <MotionSurface
                    initial={false}
                    transition={timing}
                    aria-hidden={draft.mode !== 'visual'}
                    inert={draft.mode !== 'visual'}
                    variants={policyPanelVariants}
                    animate={draft.mode === 'visual' ? 'visible' : 'visualHidden'}
                    style={toStyle({
                      position: draft.mode === 'visual' ? 'relative' : 'absolute',
                      pointerEvents: draft.mode === 'visual' ? 'auto' : 'none',
                    })}
                    class="col-start-1 row-start-1 min-w-0 w-full"
                  >
                    <PolicyTable></PolicyTable>
                  </MotionSurface><MotionSurface
                    initial={false}
                    transition={timing}
                    aria-hidden={draft.mode !== 'cedar'}
                    inert={draft.mode !== 'cedar'}
                    variants={policyPanelVariants}
                    animate={draft.mode === 'cedar' ? 'visible' : 'cedarHidden'}
                    style={toStyle({
                      position: draft.mode === 'cedar' ? 'relative' : 'absolute',
                      pointerEvents: draft.mode === 'cedar' ? 'auto' : 'none',
                    })}
                    class="col-start-1 row-start-1 min-w-0 w-full"
                  >
                    <CedarEditor locked={busy || draft.builtin || !policyPermission.allowed} owner={draft.builtin}
                    ></CedarEditor>
                  </MotionSurface>
                </div>
                <div aria-live="polite" class="mt-2 shrink-0 px-4 text-xs leading-4.5 text-muted-foreground">
                  {#if draft.parsing}Checking policy…{:else}{#if draft.diagnostics.length}<div
                        role="alert"
                        class="whitespace-pre-wrap text-destructive"
                      >
                        {draft.diagnostics.join('\n')}
                      </div>{:else}{#if draft.mode === 'cedar'}Cedar syntax is valid.{:else}{/if}{/if}{/if}
                </div>
              </section>{:else}<p class="px-4 text-xs text-muted-foreground">
                You do not have access to this role’s policies.
              </p>{/if}{/snippet}
        </PermissionRequest>{#if !locked && !draft.id}<div class="flex shrink-0 flex-wrap justify-end gap-2">
            <Button variant="primary" class="w-32 rounded-xl" loading={busy} disabled={invalid} type="submit">
              Create role
            </Button>
          </div>{:else}{/if}{#if draft.id}<PermissionRequest
            actions={['role.delete']}
            resource={{ type: 'Role', id: draft.id }}
          >
            {#snippet children(allowed)}{#if allowed}<Section title="Danger" class="shrink-0">
                  <Rows>
                    <Row label="Delete role" last>
                      <DisabledTooltip disabled={draft.builtin} reason={ownerExplanation}>
                        <Button
                          variant="ghost"
                          class="text-destructive"
                          disabled={busy || draft.builtin}
                          onclick={() => shared.pages.dialogs.openDialog('delete-role', draft.id!)}
                        >
                          Delete role
                        </Button>
                      </DisabledTooltip>
                    </Row>
                  </Rows>
                </Section>{:else}{/if}{/snippet}
          </PermissionRequest>{:else}{/if}
      </div>
    </SettingsContent>{/if}{/if}
