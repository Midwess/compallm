<script lang="ts">
  import Button from './Button.svelte'
  let { remaining = 0, onResend }: { remaining?: number; onResend?: () => void } = $props()
</script>

<p class="flex flex-wrap items-center gap-1 text-ui-12 text-muted-foreground">
  <span>Didn’t receive a code?</span><Button variant="link" disabled={remaining > 0} onclick={onResend}>
    {remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}
  </Button>
</p>
