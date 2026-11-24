<script lang="ts">
  import { Table } from '../../components/table'

  import type { SettingsMember } from './types'

  import { MemberRow } from './members'
  import { memberKey } from './members'

  let {
    label,
    members,
    complete,
    open,
    onOpenChange,
    paging,
  }: {
    label: string
    members: SettingsMember[]
    complete: boolean
    open: boolean
    onOpenChange(open: boolean): void
    paging: import('../../components/collection-types').CollectionPaging
  } = $props()
</script>

<Table.Category
  variant="directory"
  {label}
  {open}
  {onOpenChange}
  columns={6}
  count={complete || members.length ? `${members.length}${complete ? '' : ' shown'}` : undefined}
  items={members}
  getKey={memberKey}
  {paging}
>
  {#snippet children(member)}<MemberRow {member} />{/snippet}
</Table.Category>
