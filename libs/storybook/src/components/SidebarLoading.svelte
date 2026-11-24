<script lang="ts">
  import LoadingRegion from './LoadingRegion.svelte'
  import Skeleton from './Skeleton.svelte'
  let { rows = 3 }: { rows?: number } = $props()
</script>

<LoadingRegion ready={false} pending label="Loading navigation" class="grid gap-1 px-2">
  {#each Array(rows) as _, index (index)}<div class="flex h-7 items-center gap-2">
      <Skeleton><span class="inline-block size-3.5 rounded"></span></Skeleton><Skeleton>
        <span class="inline-block h-3 w-24 rounded"></span>
      </Skeleton>
    </div>{/each}
</LoadingRegion>
