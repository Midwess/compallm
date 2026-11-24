<script lang="ts">
  import Toast from '../../src/components/Toast.svelte'
  import Button from '../../src/components/Button.svelte'
  import Preview from './Preview.svelte'
  import type { ComponentProps } from 'svelte'
  let { action, onDismiss, ...props }: ComponentProps<typeof Toast> = $props()
  let visible = $state(true)
  let result = $state('')
</script>

<Preview>
  <section class="grid w-fit min-w-0 max-w-full grid-cols-1 justify-items-start gap-2">
    <Button
      onclick={() => {
        visible = true
        result = ''
      }}
    >
      Show notification
    </Button>{#if visible}<Toast
        {...props}
        onDismiss={() => {
          onDismiss()
          visible = false
        }}
        action={action
          ? {
              ...action,
              onClick: () => {
                action?.onClick()
                result = `${action?.label} selected`
                visible = false
              },
            }
          : undefined}
      />{/if}{#if result}<p role="status" class="text-xsm text-muted-foreground">{result}</p>{/if}
  </section>
</Preview>
