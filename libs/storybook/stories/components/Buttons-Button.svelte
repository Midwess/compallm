<script lang="ts">
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'
  import * as Buttons from '../../src/components/buttons'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Compact · 32 px · neutral 26 px">
    <Row>
      {#each ['primary', 'neutral', 'ghost', 'destructive', 'link'] as const as variant (variant)}<Buttons.Button {variant}>
          {variant}
        </Buttons.Button>{/each}
    </Row>
  </Sample><Sample label="States">
    <Row>
      <Buttons.Button variant="primary" loading>Continue</Buttons.Button><Buttons.Button disabled>
        Disabled
      </Buttons.Button><Buttons.Button aria-label="Add"><PlusIcon size={14}></PlusIcon></Buttons.Button>
    </Row>
  </Sample><Sample label="Standard · 40 px"><Buttons.Button size="standard">Continue</Buttons.Button></Sample><Sample
    label="Authentication · 48 px"
  >
    <Buttons.Button size="authentication" variant="primary">Continue with email</Buttons.Button>
  </Sample>
</Preview>
