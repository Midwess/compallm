<script lang="ts">
  import { SessionBrowserIcon } from './session-browser-icon'

  import { Button } from '../../components/buttons'

  import { SessionTime } from './account'

  let {
    browser,
    location,
    current = false,
    lastSeenAt,
    signedInAt,
    ipAddress,
    onRevoke,
  }: {
    browser: string
    location: string | null
    current?: boolean
    lastSeenAt?: string
    signedInAt?: string
    ipAddress?: string | null
    onRevoke?: () => void
  } = $props()
</script>

<div class="flex items-start gap-3 px-4 py-5 sm:items-center sm:gap-4">
  <span class="grid size-8 shrink-0 place-items-center rounded-md bg-elevated text-muted-foreground" aria-hidden="true">
    <SessionBrowserIcon {browser}></SessionBrowserIcon>
  </span>
  <div class="grid min-w-0 flex-1 gap-1.5">
    <div class="grid min-w-0 gap-1">
      <div class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
        <span
          class={`truncate text-xsm leading-5 font-medium${current ? ' text-[oklch(from_var(--ring)_l_c_250)]' : ''}`}
          aria-label={current ? `${browser}, current session` : undefined}
        >
          {browser}
        </span>
      </div>
    </div>
    <div class="flex min-w-0 flex-wrap items-center gap-x-3 gap-y-1.5 text-xs leading-4 text-muted-foreground">
      <span class="truncate" title={location ? `Approximate location: ${location}` : undefined}>
        {location ?? (ipAddress ? `IP address ${ipAddress}` : 'Location unavailable')}
      </span>{#if signedInAt || lastSeenAt}<span class="inline-flex flex-wrap items-center gap-x-3 gap-y-1.5">
          {#if lastSeenAt}<SessionTime label="Last active" timestamp={lastSeenAt}
            ></SessionTime>{:else}{/if}{#if signedInAt}<SessionTime label="Signed in" timestamp={signedInAt}
            ></SessionTime>{:else}{/if}
        </span>{:else}{/if}
    </div>
  </div>
  {#if onRevoke}<Button variant="ghost" class="shrink-0 px-2 text-muted-foreground" onclick={onRevoke}>
      {#if current}Log out{:else}Revoke{/if}
    </Button>{:else}{/if}
</div>
