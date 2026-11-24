<script lang="ts">
  import { FormError } from '../../components/form'
  import { FileField } from '../../components/fields'
  import { useTransition } from '../../components/motion'
  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'

  import { cx } from '../../components/shared'

  import { AvatarUploadPreview } from './shared'

  import MotionSurface from '../../components/MotionSurface.svelte'
  let {
    name,
    saved,
    field,
    label,
    disabled = false,
    disabledReason,
  }: {
    name: string
    saved: string | null
    field: 'profileUpload' | 'organizationUpload'
    label: string
    disabled?: boolean
    disabledReason?: string
  } = $props()

  let timing = $derived(useTransition('interaction'))
</script>

<MotionSurface
  data-form-field={field}
  data-ui="avatar-input"
  title={disabled ? disabledReason : undefined}
  class={cx(
    `group relative inline-flex size-10 rounded-full
focus-within:outline-2 focus-within:outline-ring`,
    disabled ? 'cursor-default' : 'cursor-pointer',
  )}
>
  <AvatarUploadPreview {name} {saved} {field}></AvatarUploadPreview>{#if !disabled}<MotionSurface
      transition={timing}
      aria-hidden="true"
      class="pointer-events-none absolute inset-0 grid place-items-center rounded-full bg-overlay/50
text-on-overlay opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 [@media(hover:none)]:opacity-100!"
    >
      <PlusIcon class="size-5" weight="bold"></PlusIcon>
    </MotionSurface>{:else}{/if}<FileField
    name={field}
    class="sr-only"
    accept="image/jpeg,image/png,image/webp"
    aria-label={label}
    {disabled}
  ></FileField><FormError name={field}></FormError>
</MotionSurface>
