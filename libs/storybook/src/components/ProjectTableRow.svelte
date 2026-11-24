<script lang="ts">
  import { Table } from './table'
  import ProjectIcon from './ProjectIcon.svelte'
  import ProjectHealth from './ProjectHealth.svelte'
  import ProjectProgress from './ProjectProgress.svelte'
  import Avatar from './Avatar.svelte'
  let {
    name,
    health = 'No updates',
    priority = 'No priority',
    lead,
    progress = 0,
  }: { name: string; health?: string; priority?: string; lead?: string; progress?: number } = $props()
</script>

<Table.Row>
  <Table.Cell>
    <span class="flex items-center gap-1.5 font-medium"><ProjectIcon /><span class="truncate">{name}</span></span>
  </Table.Cell><Table.Cell column="health"><ProjectHealth>{health}</ProjectHealth></Table.Cell><Table.Cell column="priority">
    {priority}
  </Table.Cell><Table.Cell column="lead">
    {#if lead}<Avatar name={lead} size={24} />{:else}Unassigned{/if}
  </Table.Cell><Table.Cell column="progress"><ProjectProgress value={progress} /></Table.Cell>
</Table.Row>
