<script lang="ts">
  import { Table } from '../../src/components/table'
</script>

<div class="w-full max-w-240 bg-inset p-4 text-foreground">
  <Table.Root>
    <Table.Content
      aria-label="Responsive columns"
      columns={[
        { id: 'name', class: 'w-2/5' },
        { id: 'email', class: 'w-2/5', visibilityClassName: 'hidden md:table-cell' },
        { id: 'joined', class: 'w-1/5', visibilityClassName: 'hidden lg:table-cell' },
      ]}
    >
      <Table.Header>
        <Table.Row>
          <Table.Head column="name">Name</Table.Head><Table.Head column="email">Email</Table.Head><Table.Head column="joined">
            Joined
          </Table.Head>
        </Table.Row>
      </Table.Header><Table.Category label="Members" count={1} columns={3}>
        <Table.Row>
          <Table.Cell column="name">
            Organization member with an exceptionally long display name that must truncate
          </Table.Cell><Table.Cell column="email">
            long.email.address.that.cannot.fit.in.a.narrow.column@example-organization.com
          </Table.Cell><Table.Cell column="joined">November 2026</Table.Cell>
        </Table.Row>
      </Table.Category>
    </Table.Content>
  </Table.Root>
</div>
