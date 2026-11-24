<script lang="ts">
  import { FormField } from '../../src/components/form-field'
  import { Searchbox } from '../../src/components/fields'

  import { Checkbox } from '../../src/components/selection'
</script>

<div class="grid max-w-md gap-4">
  <FormField.Root>
    <FormField.Label>Connection name</FormField.Label><FormField.Input placeholder="Enter name"
    ></FormField.Input><FormField.Hint>A name for this connection.</FormField.Hint>
  </FormField.Root><Searchbox.Root>
    <Searchbox.Input aria-label="Search models" placeholder="Search models…"></Searchbox.Input>
  </Searchbox.Root><label class="flex items-center gap-2 text-sm"><Checkbox defaultChecked></Checkbox>Enabled </label>
</div>
