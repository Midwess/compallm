<script lang="ts">
  import artworkImage from '../../assets/onboarding/midnight-penpot.webp'

  import type { Content } from '../../components/shared'
  import { toStyle } from '../page-state.svelte'
  import ContentValue from '../../components/Content.svelte'

  let {
    title,
    description,
    progress,
    children,
    action,
    footer,
    artworkColor,
  }: {
    title: Content
    description: Content
    progress: number
    children: Content
    action: Content
    footer?: Content
    artworkColor?: string | null
  } = $props()

  const id = $props.id()
</script>

<main
  aria-labelledby={id}
  class="relative isolate flex min-h-104 w-full max-w-2xl flex-col overflow-hidden
rounded-3xl bg-card px-6 pt-6 pb-4 font-sans text-foreground ring-1 ring-border ring-inset md:min-h-104"
>
  <div
    aria-hidden="true"
    class="pointer-events-none absolute inset-y-0 right-0 -z-10 isolate w-full
opacity-15 md:w-[45.833333%] md:opacity-25 dark:md:opacity-100"
    style={toStyle({
      maskImage: 'radial-gradient(ellipse 142.424% 111.905% at 157.576% 50%, black 85%, transparent 100%)',
    })}
  >
    <img
      src={artworkImage}
      alt=""
      class="absolute inset-0 size-full object-cover"
      style={toStyle({ filter: artworkColor ? 'grayscale(1)' : undefined })}
    />{#if artworkColor}<div
        class="absolute inset-0"
        style={toStyle({ backgroundColor: artworkColor, mixBlendMode: 'color' })}
      ></div>{:else}{/if}
  </div>
  <div
    role="progressbar"
    aria-label="Welcome progress"
    aria-valuemin={0}
    aria-valuemax={100}
    aria-valuenow={Math.round(progress * 100)}
    class="h-2 w-full max-w-180 shrink-0"
  >
    <div
      class="h-full rounded-full bg-primary"
      style={toStyle({
        width: `${progress * 100}%`,
        maskImage: 'linear-gradient(to right, black calc(100% - 48px), transparent)',
      })}
    ></div>
  </div>
  <header class="relative mt-4 w-full max-w-110">
    <h1 {id} class="flex min-h-8 items-center text-2xl leading-tight font-medium"><ContentValue value={title} /></h1>
    {#if description}<p
        class="flex min-h-8 max-w-100 items-center whitespace-pre-line text-sm
leading-6 text-muted-foreground"
      >
        <ContentValue value={description} />
      </p>{:else}{/if}
  </header>
  <div class="relative mt-6 flex w-full max-w-96 flex-1 flex-col justify-start"><ContentValue value={children} /></div>
  <footer class="relative mt-auto w-full max-w-96 pt-4"><ContentValue value={action} /><ContentValue value={footer} /></footer>
</main>
