<script lang="ts">
  import * as Buttons from '../../src/components/buttons'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Link tones">
    <Buttons.TextLink href="#different-email">Use a different email</Buttons.TextLink><Buttons.TextLink
      tone="muted"
      href="#help"
    >
      Learn more
    </Buttons.TextLink><Buttons.TextLink disabled href="#disabled">Unavailable</Buttons.TextLink>
  </Sample>
</Preview>
