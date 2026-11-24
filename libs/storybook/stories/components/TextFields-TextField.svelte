<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Empty">
    <Fields.TextField aria-label="Empty text" placeholder="Enter a value…"></Fields.TextField>
  </Sample><Sample label="Filled">
    <Fields.TextField aria-label="Filled text" value="Application name"></Fields.TextField>
  </Sample><Sample label="Error">
    <Fields.TextField aria-label="Invalid text" aria-invalid="true" value="Invalid value" aria-describedby="text-destructive"
    ></Fields.TextField>
    <p id="text-destructive" class="text-xsm leading-5 text-destructive">Check this value.</p>
  </Sample><Sample label="Disabled">
    <Fields.TextField aria-label="Disabled text" disabled placeholder="Enter a value…"></Fields.TextField>
  </Sample><Sample label="Constant">
    <Fields.TextField variant="constant" aria-label="Account email" value="alex@example.com"></Fields.TextField>
  </Sample>
</Preview>
