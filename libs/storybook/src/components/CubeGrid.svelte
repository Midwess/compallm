<script lang="ts">
  import { animate } from 'motion'
  import { reducedMotion } from './motion'
  let { stopped = false, reducedMotion: forceReducedMotion = false }: { stopped?: boolean; reducedMotion?: boolean } = $props()
  function cube(node: HTMLElement, index: number) {
    let stop = () => {}
    $effect(() => {
      stop()
      if (stopped || forceReducedMotion || reducedMotion()) return
      const controls = animate(
        node,
        { scale: [1, 0, 1, 1] },
        {
          duration: 1.3,
          ease: 'easeInOut',
          repeat: Infinity,
          times: [0, 0.35, 0.7, 1],
          delay: (2 - Math.floor(index / 3) + (index % 3)) * 0.1,
        },
      )
      stop = () => {
        controls.stop()
        node.style.transform = ''
      }
      return stop
    })
    return {
      destroy() {
        stop()
      },
    }
  }
</script>

<span aria-hidden="true" class="grid size-8 grid-cols-3 grid-rows-3 text-muted-foreground/60">
  {#each Array(9) as _, index (index)}<span class="bg-current" use:cube={index}></span>{/each}
</span>
