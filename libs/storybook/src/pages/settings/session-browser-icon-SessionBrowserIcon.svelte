<script lang="ts">
  import BrowserIcon from 'phosphor-svelte/lib/BrowserIcon'
  import CompassIcon from 'phosphor-svelte/lib/CompassIcon'

  import { paths } from './session-browser-icon'

  let {
    browser,
  }: {
    browser: string
  } = $props()

  let brand = $derived(
    /chrome|chromium/i.test(browser) ? 'chrome' : /\barc\b/i.test(browser) ? 'arc' : /brave/i.test(browser) ? 'brave' : null,
  )
</script>

{#if !brand}{@const Icon = /safari/i.test(browser) ? CompassIcon : BrowserIcon}<Icon size={16} aria-hidden="true"
  ></Icon>{:else}<svg width={16} height={16} viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
    {#each paths[brand] as d, index (index)}<path {d}></path>{/each}
  </svg>{/if}
