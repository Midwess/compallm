<script lang="ts">
  import AccessLayout from './AccessLayout.svelte'
  import AccessPanel from './AccessPanel.svelte'
  let { mode = 'login' }: { mode?: 'login' | 'signup' } = $props()
</script>

<AccessLayout><AccessPanel intent={mode} /></AccessLayout>
