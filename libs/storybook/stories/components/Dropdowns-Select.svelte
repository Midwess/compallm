<script lang="ts">
  import * as Menus from '../../src/components/dropdowns'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'

  const options = [
    { value: 'typescript', label: 'TypeScript' },
    { value: 'rust', label: 'Rust' },
    { value: 'python', label: 'Python' },
    { value: 'unavailable', label: 'Unavailable', disabled: true },
  ]
</script>

{#snippet dot()}<span aria-hidden="true" class="size-1.5 rounded-full bg-muted-foreground"></span>{/snippet}
{#snippet count()}<span aria-hidden="true" class="text-muted-foreground">1</span>{/snippet}
<Preview>
  <Sample label="Custom select"><Menus.Select.Control aria-label="Language" {options}></Menus.Select.Control></Sample><Sample
    label="Options with icons"
  >
    <Menus.Select.Control
      aria-label="Badge style"
      options={[
        {
          value: 'dot',
          label: 'Dot',
          icon: dot,
        },
        {
          value: 'count',
          label: 'Count',
          icon: count,
        },
      ]}
    ></Menus.Select.Control>
  </Sample><Sample label="Disabled">
    <Menus.Select.Control aria-label="Disabled language" {options} disabled></Menus.Select.Control>
  </Sample>
</Preview>
