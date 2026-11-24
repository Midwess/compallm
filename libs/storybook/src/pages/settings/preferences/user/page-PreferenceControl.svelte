<script lang="ts">
  import { FormError } from '../../../../components/form'

  import { Select } from '../../../../components/dropdowns'
  import { Switch } from '../../../../components/selection'

  import { Row } from '../../shared'

  import type { Preference } from './page'

  let {
    item,
    last = false,
  }: {
    item: Preference
    last?: boolean
  } = $props()

  const id = $props.id()
</script>

<Row description={item.description} {last}>
  {#snippet label()}{#if item.options}{item.label}{:else}<label for={id}>
        {item.label}
      </label>{/if}{/snippet}{#if item.options}<Select.Control
      aria-label={item.label}
      name={`draft.${item.key}`}
      class="w-44"
      options={item.options}
    ></Select.Control>{:else}<Switch {id} name={`draft.${item.key}`}></Switch>{/if}<FormError name={`draft.${item.key}`}
  ></FormError>
</Row>
