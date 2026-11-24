<script lang="ts">
  import { codeAppearance } from './code-appearance.svelte'
  const appearance = codeAppearance()

  import MultilineField from './MultilineField.svelte'
  import { codeTokens } from './code-tokens.svelte'
  import { fieldModel } from './form-context.svelte'
  import { cx } from './shared'
  import type { Snippet } from 'svelte'
  export interface CodeEditorProps {
    name?: string
    value?: string
    onValueChange?: (value: string) => void
    language?: string
    label?: string
    theme?: 'dark' | 'light'
    fontSize?: number
    fontFamily?: string
    readOnly?: boolean
    invalid?: boolean
    actions?: Snippet
    showToolbar?: boolean
    class?: string
    'aria-label'?: string
  }
  let {
    name,
    value = $bindable(''),
    onValueChange,
    language = 'typescript',
    label,
    theme,
    fontSize,
    fontFamily,
    readOnly = false,
    invalid = false,
    actions,
    showToolbar = true,
    class: className,
    'aria-label': ariaLabel,
  }: CodeEditorProps = $props()
  const form = fieldModel(() => name)
  let current = $derived(String(form && name ? (form.get(name) ?? '') : value))
  let effectiveTheme = $derived(theme ?? appearance().theme)
  let effectiveSize = $derived(fontSize ?? appearance().fontSize)
  let effectiveFont = $derived(fontFamily ?? appearance().fontFamily)
  const tokens = codeTokens(
    () => current,
    () => language,
    () => effectiveTheme,
  )
  let content: HTMLPreElement
  let gutter: HTMLDivElement
  let style = $derived(
    `font-size:${effectiveSize}px;line-height:${(effectiveSize * 5) / 3}px;font-family:${effectiveFont};tab-size:2`,
  )
  const newline = '\n'
</script>

<div
  data-slot="code-editor"
  data-form-field={name}
  data-code-theme={effectiveTheme}
  class={cx('flex min-w-0 w-full flex-col overflow-hidden rounded-md bg-code-background text-code-foreground', className)}
>
  {#if showToolbar}<div class="flex min-h-10 shrink-0 flex-wrap items-center justify-end gap-2 px-4 py-2 text-xs">
      {@render actions?.()}<span
        class="pointer-events-none rounded-lg bg-elevated px-2.5 py-1 font-sans text-xs leading-4 text-foreground"
      >
        {language}
      </span>
    </div>{/if}
  <div
    class="relative min-h-32 w-full"
    style={`height:${Math.max(128, ((current.split('\n').length + 1) * effectiveSize * 5) / 3 + 16)}px`}
  >
    <div
      aria-hidden="true"
      class="pointer-events-none absolute inset-0 overflow-hidden font-mono [font-variant-ligatures:none]"
      {style}
    >
      <pre bind:this={content} class="absolute top-2 left-9 m-0 w-max min-w-full font-mono" {style}><code
          >{#each tokens() as line, index (index)}<span class="block whitespace-pre"
              >{#each line as token, j (j)}<span style:color={token.color}>{token.content}</span>{/each}{newline}</span
            >{/each}</code
        ></pre>
      <div class="absolute inset-y-0 left-0 w-7 overflow-hidden bg-code-background">
        <div bind:this={gutter} class="pt-2 pr-0.5 text-right text-code-gutter">
          {#each tokens() as _, index (index)}<div>{index + 1}</div>{/each}
        </div>
      </div>
    </div>
    <MultilineField
      {name}
      aria-label={label ?? ariaLabel ?? 'Source code'}
      aria-invalid={invalid}
      value={current}
      readonly={readOnly}
      spellcheck={false}
      autocapitalize="off"
      autocorrect="off"
      wrap="off"
      class="absolute inset-0 h-full min-h-0 w-full resize-none rounded-none border-0 bg-transparent py-2 pl-9 pr-4 font-mono text-transparent caret-foreground selection:bg-ring/25 [font-variant-ligatures:none] [-webkit-text-fill-color:transparent]"
      style={style + ';caret-color:var(--code-foreground)'}
      oninput={(e) => {
        value = e.currentTarget.value
        onValueChange?.(value)
      }}
      onscroll={(e) => {
        const { scrollTop, scrollLeft } = e.currentTarget
        content.style.transform = `translate(${-scrollLeft}px,${-scrollTop}px)`
        gutter.style.transform = `translateY(${-scrollTop}px)`
      }}
    />
  </div>
</div>
