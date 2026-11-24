<script lang="ts">
  import { LoadingRegion } from '../../components/loading'

  import { List } from '../../components/list'

  import { Button } from '../../components/buttons'

  import { Section, Rows } from './shared'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getAuthController, getSettingsController } from '../../context'
  import { SessionItem, CurrentSession } from './account'

  const __settings = getSettingsController()
  const controller = untrack(() => getAuthController())
  const settings = untrack(() => getSettingsController())
  let otherSessions = $derived(readStore(__settings.store, (state) => state.otherSessions))
  let ready = $derived(readStore(__settings.store, (state) => state.sessionsLoaded ?? !state.sessionsLoading))
  let loading = $derived(readStore(__settings.store, (state) => state.sessionsLoading))
  let hasMore = $derived(readStore(__settings.store, (state) => state.sessionsHasNextPage))
  let error = $derived(readStore(__settings.store, (state) => state.sessionsError))
  let location = $derived(readStore(__settings.store, (state) => state.sessionLocation))
</script>

<Section title="Sessions" description="Devices logged into your account">
  {#snippet action()}{#if settings.reloadSessions}<Button variant="ghost" onclick={settings.reloadSessions}>
        Refresh
      </Button>{:else}{undefined}{/if}{/snippet}
  <div class="grid gap-3">
    <Rows class="shadow-control">
      {#if controller}<CurrentSession {controller}></CurrentSession>{:else}<SessionItem
          browser="Unknown browser"
          location={null}
          current
          onRevoke={settings.signOut}
        ></SessionItem>{/if}
    </Rows><LoadingRegion {ready} pending={loading} label="Loading sessions">
      {#if !ready}<Rows>
          <List.Root>
            <List.Items items={[]} getKey={() => ''}>{#snippet children(_item, _index)}<span></span>{/snippet}</List.Items>
          </List.Root>
        </Rows>{:else}{/if}{#if otherSessions.length}<Rows class="shadow-control">
          <div class="flex min-h-16 items-center justify-between gap-4 border-b border-border p-4">
            <span class="text-xsm font-medium">
              {otherSessions.length} other {#if otherSessions.length === 1}session{:else}sessions{/if}{#if hasMore}
                shown{:else}{/if}
            </span><Button variant="ghost" class="px-3" onclick={() => settings.pages.dialogs.openDialog('revoke-sessions')}>
              Revoke all
            </Button>
          </div>
          <List.Root>
            <List.Items
              items={otherSessions}
              getKey={(session) => session.id}
              class="m-0 list-none p-0"
              paging={{ hasMore, loading, error, onLoadMore: () => settings.loadMoreSessions?.() }}
            >
              {#snippet children(session)}<div
                  class={session.id === otherSessions[0].id
                    ? undefined
                    : 'relative before:absolute before:inset-x-4 before:top-0 before:border-t before:border-border'}
                >
                  <SessionItem
                    browser={session.browser}
                    location={session.location}
                    ipAddress={session.ipAddress}
                    lastSeenAt={session.lastSeenAt}
                    signedInAt={session.signedInAt}
                    onRevoke={settings.revokeSession ? () => settings.revokeSession!(session.id) : undefined}
                  ></SessionItem>
                </div>{/snippet}
            </List.Items>
          </List.Root>
        </Rows>{:else}{/if}
    </LoadingRegion>{#if error}<div class="flex items-center justify-between gap-3 px-4 text-xs text-muted-foreground">
        Couldn’t load sessions. <Button variant="ghost" onclick={() => settings.reloadSessions?.()}>Retry</Button>
      </div>{:else}{/if}{#if hasMore && !loading && !error}<Button
        variant="ghost"
        onclick={() => settings.loadMoreSessions?.()}
      >
        Load more sessions
      </Button>{:else}{/if}{#if location || otherSessions.some((session) => session.location)}<a
        href="https://db-ip.com"
        target="_blank"
        rel="noreferrer"
        class="justify-self-end px-4 text-xs text-muted-foreground hover:text-foreground"
      >
        IP Geolocation by DB-IP
      </a>{:else}{/if}
  </div>
</Section>
