<script lang="ts">
  import * as Navigation from '../../src/components/navigation'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Breadcrumb trail">
    <Navigation.BreadcrumbTrail items={[{ label: 'Projects', href: '#projects' }, { label: 'Compallm' }]}
    ></Navigation.BreadcrumbTrail>
  </Sample>
</Preview>
