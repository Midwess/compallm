<script lang="ts">
  import * as Buttons from '../../src/components/buttons'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Signup">
    <Buttons.AccountPrompt>
      {#snippet action()}<Buttons.TextLink href="#signin">Sign in</Buttons.TextLink>{/snippet}Already have an account?
    </Buttons.AccountPrompt>
  </Sample><Sample label="Signin">
    <Buttons.AccountPrompt>
      {#snippet action()}<Buttons.TextLink href="#signup">Sign up</Buttons.TextLink>{/snippet}New to Compallm?
    </Buttons.AccountPrompt>
  </Sample>
</Preview>
