<script lang="ts">
  import { untrack } from 'svelte'
  import { Switch, CheckboxOption, Checkbox, Radio, Button } from '../../src'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
  let { scenario = 'TransitionsAndReset' }: { scenario?: string } = $props()
  let checked = $state(untrack(() => scenario === 'RestoredInStrictMode'))
</script>

<Preview>
  <Sample
    label={scenario === 'TransitionsAndReset' ? 'Controlled and native selection transitions' : 'Loaded selection state'}
    wide
  >
    {#if scenario === 'TransitionsAndReset'}<form class="grid gap-4">
        <label class="flex items-center gap-2 text-xsm"><Switch bind:checked />Controlled switch</label><Button
          onclick={() => (checked = !checked)}
        >
          Toggle controlled value
        </Button><CheckboxOption defaultChecked>Resettable checkbox</CheckboxOption><Row>
          <label class="flex items-center gap-2 text-xsm">
            <Radio name="motion-choice" value="first" defaultChecked /> First
          </label><label class="flex items-center gap-2 text-xsm"><Radio name="motion-choice" value="second" /> Second</label>
        </Row><Button type="reset">Reset native selections</Button>
      </form>
    {:else}<label class="flex items-center gap-2 text-xsm">
        <Switch bind:checked />Saved permission: {checked ? 'enabled' : 'disabled'}
      </label><Row>
        <Checkbox aria-label="Saved checked value" defaultChecked /><Checkbox
          aria-label="Saved mixed value"
          indeterminate
        /><Radio aria-label="Saved radio value" defaultChecked />
      </Row><Button onclick={() => (checked = true)}>Restore enabled permission</Button>{/if}
  </Sample>
</Preview>
