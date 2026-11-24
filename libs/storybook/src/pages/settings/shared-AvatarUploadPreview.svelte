<script lang="ts">
  import { ImageUploadInput } from '@compallm/contract/validation'
  import { Avatar } from '../../components/content'

  import { untrack } from 'svelte'

  import { readForm } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  let {
    name,
    saved,
    field,
  }: {
    name: string
    saved: string | null
    field: 'profileUpload' | 'organizationUpload'
  } = $props()

  const settings = getSettingsController()
  let file = $derived(readForm<File | null>(settings.store.form, field))
  let preview: {
    file: File
    url: string
  } | null = $state(null)
  $effect(() => {
    void [file]
    return untrack(() => {
      if (
        !file ||
        !ImageUploadInput.safeParse({
          size: file.size,
          mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
        }).success
      )
        return
      const reader = new FileReader()
      reader.onload = () => (preview = { file, url: String(reader.result) })
      reader.readAsDataURL(file)
      return () => {
        reader.onload = null
        reader.abort()
      }
    })
  })
</script>

<Avatar {name} src={file && preview?.file === file ? preview.url : (saved ?? undefined)}></Avatar>
