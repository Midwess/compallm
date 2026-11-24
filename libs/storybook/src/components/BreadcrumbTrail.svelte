<script lang="ts">
  import CaretRightIcon from 'phosphor-svelte/lib/CaretRightIcon'
  let { items, label = 'Breadcrumb' }: { items: { label: string; href?: string }[]; label?: string } = $props()
</script>

<nav aria-label={label}>
  <ol
    class="flex list-none items-center gap-1.5 p-0 text-xsm [&>li]:flex [&>li]:min-w-0 [&>li]:items-center [&>li]:gap-1.5 [&_a]:text-muted-foreground [&_a]:focus-visible:outline-2 [&_a]:focus-visible:outline-offset-2 [&_a]:focus-visible:outline-ring"
  >
    {#each items as item, index (index)}<li>
        {#if index > 0}<CaretRightIcon size={12} aria-hidden="true" />{/if}{#if item.href}<a
            href={item.href}
            aria-current={index === items.length - 1 ? 'page' : undefined}
          >
            {item.label}
          </a>{:else}<span aria-current={index === items.length - 1 ? 'page' : undefined}>{item.label}</span>{/if}
      </li>{/each}
  </ol>
</nav>
