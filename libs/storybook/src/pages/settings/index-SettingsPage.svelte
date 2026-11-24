<script lang="ts">
  import ListIcon from 'phosphor-svelte/lib/ListIcon'
  import { Button } from '../../components/buttons'
  import { Toast } from '../../components/feedback'
  import { SettingsNavigation } from './sidebar'
  import { SettingsDialogs } from './dialogs'

  import { cx } from '../../components/shared'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { SharedSettingsForm } from './index'

  const __settings = getSettingsController()
  let open = $state(false)
  const c = untrack(() => getSettingsController())
  let membersPage = $derived(readStore(__settings.store, (s) => s.location.page === 'members'))
  let notice = $derived(readStore(__settings.store, (s) => s.notice))
  let errorToast = $derived(readStore(__settings.store, (s) => s.errorToast))
  $effect(() => {
    void [c]
    return untrack(() => {
      const sources = [
        { store: c.store, clear: () => c.store.setState({ error: null }) },
        { store: c.role.store, clear: () => c.role.store.setState({ error: null }) },
        { store: c.group.store, clear: () => c.group.store.setState({ error: null }) },
      ]
      const subscriptions = sources.map(({ store, clear }) => {
        const report = () => {
          const error = store.getState().error
          if (!error) return
          clear()
          c.apiError(
            { info: { message: error } },
            store === c.store
              ? undefined
              : () => {
                  void store.form.retry()
                },
          )
        }
        report()
        return store.subscribe(report)
      })
      return () => subscriptions.forEach((unsubscribe) => unsubscribe())
    })
  })
</script>

<div class="relative flex h-dvh min-h-0 overflow-hidden bg-background font-sans text-foreground">
  {#if open}<button
      type="button"
      aria-label="Close settings navigation"
      class="fixed inset-0 z-30 bg-background/60 md:hidden"
      onclick={() => (open = false)}
    ></button>{:else}{/if}
  <div class={cx('fixed inset-y-0 left-0 z-40 w-61 bg-background md:static md:block', open ? 'block' : 'hidden')}>
    <SettingsNavigation onNavigate={() => (open = false)}></SettingsNavigation>
  </div>
  <main
    class={cx(
      `@container relative mr-2 mb-2 ml-2 min-h-0 min-w-0 flex-1 overflow-x-hidden overflow-y-auto
overscroll-y-none rounded-xl border-(length:--border-hairline) border-divider bg-surface pr-2 scrollbar-gutter-stable md:ml-0`,
      membersPage ? 'mt-2' : 'mt-4',
    )}
    aria-label="Settings content"
  >
    <button
      type="button"
      aria-label="Open settings navigation"
      onclick={() => (open = true)}
      class="absolute top-4 left-4 z-10 rounded p-1 text-muted-foreground hover:text-foreground md:hidden"
    >
      <ListIcon class="size-4"></ListIcon>
    </button><SharedSettingsForm></SharedSettingsForm>
  </main>
  {#if notice}<div
      role="status"
      class="fixed right-6 bottom-6 z-50 flex max-w-[calc(100vw-3rem)] items-center gap-4
rounded-lg border border-border bg-elevated px-4 py-3 text-xsm shadow-menu"
    >
      {notice}<Button variant="ghost" class="h-6 p-0" onclick={() => c.dismissNotice()}>Dismiss</Button>
    </div>{:else}{/if}{#if errorToast}<div class="fixed right-6 bottom-6 z-50 max-w-[calc(100vw-3rem)]">
      <Toast
        tone="error"
        message={errorToast.message}
        onDismiss={c.dismissErrorToast}
        action={errorToast.retry
          ? {
              label: 'Retry',
              onClick: () => {
                c.dismissErrorToast()
                errorToast.retry?.()
              },
            }
          : undefined}
      ></Toast>
    </div>{:else}{/if}<SettingsDialogs></SettingsDialogs>
</div>
