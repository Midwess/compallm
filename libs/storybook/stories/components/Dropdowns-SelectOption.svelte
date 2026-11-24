<script lang="ts">
  import * as Menus from '../../src/components/dropdowns'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Default and selected">
    <Menus.Select.Root
      value="selected"
      items={[
        { value: 'default', label: 'Default option' },
        { value: 'selected', label: 'Selected option' },
      ]}
    >
      <Menus.Select.Trigger aria-label="Option selection" class="w-60">
        <Menus.Select.Value></Menus.Select.Value>
      </Menus.Select.Trigger><Menus.Select.Content aria-label="Option selection">
        <Menus.SelectOption value="default">Default option</Menus.SelectOption><Menus.SelectOption value="selected">
          Selected option
        </Menus.SelectOption>
      </Menus.Select.Content>
    </Menus.Select.Root>
  </Sample>
</Preview>
