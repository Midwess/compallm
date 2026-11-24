<script lang="ts">
  import { preferenceOptions, preferenceLabel } from './values'

  import { CodePreview } from '../../../../components/content'
  import { Popup } from '../../../../components/popup'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'
  import { SettingsRow } from '../../../../components/content'
  import { Rows } from '../../shared'
  import { untrack } from 'svelte'
  import { readStore } from '../../../page-state.svelte'
  import { getUserPreferenceController, getSettingsController } from '../../../../context'
  import { PreferenceControl } from './page'

  import ContentValue from '../../../../components/Content.svelte'

  const __preferences = getUserPreferenceController()
  const __settings = getSettingsController()
  const c = untrack(() => getUserPreferenceController())
  let fontSize = $derived(readStore(__preferences.store, (s) => s.draft.codeFontSize, ['draft.codeFontSize']))
  let font = $derived(readStore(__preferences.store, (s) => s.draft.codeFont, ['draft.codeFont']))
  let language = $derived(readStore(__preferences.store, (s) => s.draft.codeLanguage, ['draft.codeLanguage']))
  let examples = $derived(readStore(__settings.store, (s) => s.codeExamples))
  let example = $derived(examples[language] ?? { code: '' })
  const readAppearance = codeAppearance()
  let appearance = $derived(readAppearance())
  import { codeAppearance } from '../../../../components/code-appearance.svelte'
</script>

<Rows>
  <PreferenceControl
    item={{
      key: 'codeTheme',
      label: 'Code theme',
      description: 'Matches your interface theme unless overridden.',
      options: preferenceOptions.codeTheme,
    }}
  ></PreferenceControl><SettingsRow label="Font" class="min-h-15 bg-transparent px-4 py-3">
    <Popup.Root>
      <Popup.Trigger aria-label="Code font" class="h-8 w-auto gap-2 px-0 text-xs text-muted-foreground">
        {preferenceLabel(fontSize)}, Regular, {preferenceLabel(font)}<CaretDownIcon aria-hidden="true" class="size-3"
        ></CaretDownIcon>
      </Popup.Trigger><Popup.Content aria-label="Code font settings">
        <Popup.Sub>
          <Popup.SubTrigger>Font size</Popup.SubTrigger><Popup.SubContent aria-label="Font size">
            {#each preferenceOptions.codeFontSize as { value, label } (value)}<Popup.Item
                onSelect={() => c.store.form.change('draft.codeFontSize', value)}
              >
                <ContentValue value={label} />
              </Popup.Item>{/each}
          </Popup.SubContent>
        </Popup.Sub><Popup.Sub>
          <Popup.SubTrigger>Font</Popup.SubTrigger><Popup.SubContent aria-label="Font">
            {#each preferenceOptions.codeFont as { value, label } (value)}<Popup.Item
                onSelect={() => c.store.form.change('draft.' + 'codeFont', value)}
              >
                <ContentValue value={label} />
              </Popup.Item>{/each}
          </Popup.SubContent>
        </Popup.Sub>
      </Popup.Content>
    </Popup.Root>
  </SettingsRow>
  <div class="px-2 pb-2">
    <CodePreview
      {...appearance}
      code={example.code}
      lineChanges={example.lineChanges}
      {language}
      onLanguageChange={(value) => c.store.form.change('draft.' + 'codeLanguage', value)}
    ></CodePreview>
  </div>
</Rows>
