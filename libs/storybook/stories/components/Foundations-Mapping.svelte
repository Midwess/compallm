<script lang="ts">
  import { Table } from '../../src/components/table'

  import { SettingsPageTitle } from '../../src/components/settings'
  import { foundationFamilies, penpotFoundationUrl } from './foundation-mapping'
  const linkClassName = `text-ring underline-offset-4 hover:underline
focus-visible:outline-2 focus-visible:outline-ring`
</script>

<div class="grid gap-6 p-6">
  <div class="grid gap-2">
    <SettingsPageTitle>Design foundations</SettingsPageTitle>
    <p class="text-xsm leading-5 text-muted-foreground">
      Penpot foundation families map to shared components and their existing Storybook previews.
    </p>
  </div>
  <Table.Root>
    <Table.Content class="min-w-3xl text-xsm" aria-label="Penpot to Storybook foundation mapping">
      <Table.Header>
        <Table.Row>
          <Table.Head class="w-3xs">Penpot family</Table.Head><Table.Head>Library source</Table.Head><Table.Head class="w-32">
            Preview
          </Table.Head>
        </Table.Row>
      </Table.Header><Table.Body>
        {#each foundationFamilies as family (family.board)}<Table.Row>
            <Table.Cell>
              <a href={penpotFoundationUrl(family.board)} target="_blank" rel="noreferrer" class={linkClassName}>
                {family.name}
              </a>
            </Table.Cell><Table.Cell class="wrap-break-word text-muted-foreground">{family.source}</Table.Cell><Table.Cell>
              <a href={`/?path=/story/${family.story}`} target="_top" class={linkClassName}>Open story </a>
            </Table.Cell>
          </Table.Row>{/each}
      </Table.Body>
    </Table.Content>
  </Table.Root>
</div>
