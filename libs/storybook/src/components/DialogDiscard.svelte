<script lang="ts">
  import type { Snippet } from 'svelte'
  import { getDialog } from './dialog-context'
  import Root from './DialogRoot.svelte'
  import Content from './AlertDialogContent.svelte'
  import Header from './DialogHeader.svelte'
  import Title from './DialogTitle.svelte'
  import Body from './DialogBody.svelte'
  import Description from './DialogDescription.svelte'
  import Footer from './DialogFooter.svelte'
  import Button from './Button.svelte'
  let {
    dirty = true,
    children,
    request,
  }: {
    dirty?: boolean
    children?: Snippet
    request?: { keep(): void; discard(): void }
  } = $props()
  const scope = getDialog()
  let confirm = $state(false)
  $effect(() => {
    scope.registerDiscard(
      dirty
        ? () => {
            confirm = true
          }
        : undefined,
    )
    return () => scope.registerDiscard(undefined)
  })
  function keep() {
    confirm = false
    request?.keep()
  }
  function discard() {
    confirm = false
    if (request) request.discard()
    else scope.setOpen(false)
  }
</script>

<Root
  open={confirm || !!request}
  onOpenChange={(open) => {
    if (!open) keep()
  }}
>
  <Content>
    <Header><Title>Discard changes?</Title></Header>
    <Body>
      <Description>
        {#if children}{@render children()}{:else}Your unsaved changes will be lost.{/if}
      </Description>
    </Body>
    <Footer>
      <Button autofocus class="h-8 px-3 text-xs" onclick={keep}>Keep editing</Button>
      <Button variant="destructive" class="h-8 px-3 text-xs" onclick={discard}>Discard changes</Button>
    </Footer>
  </Content>
</Root>
