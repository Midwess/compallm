<script lang="ts">
  import { codeAppearance } from './code-appearance.svelte'
  const appearance = codeAppearance()

  import { codeTokens } from './code-tokens.svelte'
  import { cx } from './shared'
  import Dropdown from './Dropdown.svelte'
  export interface CodePreviewProps {
    code: string
    language?: string
    theme?: 'dark' | 'light'
    fontSize?: number
    fontFamily?: string
    lineChanges?: Record<number, 'added' | 'removed'>
    onLanguageChange?: (language: string) => void
    class?: string
  }
  let {
    code,
    language = 'text',
    theme,
    fontSize,
    fontFamily,
    lineChanges = {},
    onLanguageChange,
    class: className,
  }: CodePreviewProps = $props()
  let effectiveTheme = $derived(theme ?? appearance().theme)
  let effectiveSize = $derived(fontSize ?? appearance().fontSize)
  let effectiveFont = $derived(fontFamily ?? appearance().fontFamily)
  const tokens = codeTokens(
    () => code,
    () => language,
    () => effectiveTheme,
  )
  const labels: Record<string, string> = {
    typescript: 'TypeScript',
    javascript: 'JavaScript',
    json: 'JSON',
    bash: 'Bash',
    cedar: 'Cedar',
  }
  const newline = '\n'
</script>

<figure
  data-slot="code-block"
  data-code-theme={effectiveTheme}
  class={cx('relative m-0 overflow-hidden rounded-md bg-code-background text-code-foreground', className)}
>
  <div class="absolute top-2 right-5 z-10 flex items-center gap-2 text-xs">
    {#if onLanguageChange}<Dropdown
        value={language.toLowerCase()}
        onValueChange={onLanguageChange}
        options={Object.entries(labels).map(([value, label]) => ({ value, label }))}
        label="Code language"
      />{:else}<span class="pointer-events-none rounded-lg bg-elevated px-2.5 py-1 font-sans text-xs leading-4 text-foreground">
        {labels[language.toLowerCase()] ?? language}
      </span>{/if}
  </div>
  <pre
    class="m-0 overflow-auto font-mono [font-variant-ligatures:none]"
    style={`font-size:${effectiveSize}px;line-height:${(effectiveSize * 5) / 3}px;font-family:${effectiveFont}`}><code
      class="block min-w-full w-max"
      >{#each tokens() as line, index (index)}{@const change = lineChanges[index + 1]}<span
          data-change={change}
          class="flex min-w-full border-l border-transparent data-[change=added]:border-code-added-indicator data-[change=added]:bg-code-added data-[change=removed]:border-code-removed-indicator data-[change=removed]:bg-code-removed"
          ><span
            aria-hidden="true"
            class={cx(
              'sticky left-0 w-7 shrink-0 select-none pr-0.5 text-right text-code-gutter',
              change === 'added' && 'text-code-added-indicator',
              change === 'removed' && 'text-code-removed-indicator',
            )}
            >{change === 'removed'
              ? '−'
              : index +
                1 -
                Object.keys(lineChanges).filter((key) => Number(key) <= index + 1 && lineChanges[Number(key)] === 'removed')
                  .length}</span
          ><span class="block flex-1 whitespace-pre pl-2 pr-4"
            >{#each line as token, j (j)}<span style:color={token.color}>{token.content}</span>{/each}{newline}</span
          ></span
        >{/each}</code
    ></pre>
</figure>
