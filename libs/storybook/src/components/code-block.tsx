import { useEffect, useRef, useState, type CSSProperties, type ReactNode } from 'react'
import type { ThemedToken } from 'shiki/core'
import { useFormField } from './form'
import { MultilineField } from './fields'
import { Select } from './dropdowns'
import { cx } from './shared'
import { useSavedCodeAppearance } from './code-appearance'

type CodeTheme = 'dark' | 'light'
type Change = 'added' | 'removed'
const labels: Record<string, string> = {
  typescript: 'TypeScript',
  javascript: 'JavaScript',
  json: 'JSON',
  bash: 'Bash',
  cedar: 'Cedar',
}

function useTokens(code: string, language: string, theme: CodeTheme) {
  const [result, setResult] = useState<{ code: string; language: string; theme: CodeTheme; tokens: ThemedToken[][] }>()
  useEffect(() => {
    let active = true
    const timer = setTimeout(() => {
      void import('./code-highlighter')
        .then((module) => module.highlightCode(code, language, theme))
        .then((tokens) => {
          if (active) setResult({ code, language, theme, tokens })
        })
        .catch(() => {
          // The canonical text remains readable if syntax highlighting is unavailable.
        })
    }, 80)
    return () => {
      active = false
      clearTimeout(timer)
    }
  }, [code, language, theme])
  return result?.code === code && result.language === language && result.theme === theme
    ? result.tokens
    : code.split('\n').map((content) => [{ content, offset: 0 }])
}

function Language({
  language,
  onChange,
  actions,
  header = false,
}: {
  language: string
  onChange?: (language: string) => void
  actions?: ReactNode
  header?: boolean
}) {
  const name = labels[language.toLowerCase()] ?? language
  return (
    <div
      className={
        header
          ? 'flex min-h-10 shrink-0 flex-wrap items-center justify-end gap-2 px-4 py-2 text-xs'
          : 'absolute top-2 right-5 z-10 flex items-center gap-2 text-xs'
      }
    >
      {actions}
      {onChange ? (
        <Select
          aria-label="Code language"
          value={language.toLowerCase()}
          onValueChange={onChange}
          options={Object.entries(labels).map(([value, label]) => ({ value, label }))}
          className="w-25 [&>button]:h-6 [&>button]:rounded-lg [&>button]:px-2.5
[&>button]:text-xs [&>button]:leading-none [&>button]:shadow-none"
        />
      ) : (
        <span className="pointer-events-none rounded-lg bg-elevated px-2.5 py-1 font-sans text-xs leading-4 text-foreground">
          {name}
        </span>
      )}
    </div>
  )
}

function Tokens({ tokens }: { tokens: { content: string; color?: string }[] }) {
  return (
    <>
      {tokens.map((token, i) => (
        <span key={i} style={{ color: token.color }}>
          {token.content}
        </span>
      ))}
    </>
  )
}

export interface CodePreviewProps {
  code: string
  language?: string
  theme?: CodeTheme
  fontSize?: number
  fontFamily?: string
  lineChanges?: Record<number, Change>
  onLanguageChange?: (language: string) => void
  className?: string
}

export function CodePreview({
  code,
  language = 'text',
  theme: suppliedTheme,
  fontSize: suppliedFontSize,
  fontFamily: suppliedFontFamily,
  lineChanges = {},
  onLanguageChange,
  className,
}: CodePreviewProps) {
  const saved = useSavedCodeAppearance()
  const theme = suppliedTheme ?? saved.theme
  const fontSize = suppliedFontSize ?? saved.fontSize
  const fontFamily = suppliedFontFamily ?? saved.fontFamily
  const lines = useTokens(code, language, theme)
  let number = 0
  return (
    <figure
      data-slot="code-block"
      data-code-theme={theme}
      className={cx('relative m-0 overflow-hidden rounded-md bg-code-background text-code-foreground', className)}
    >
      <Language language={language} onChange={onLanguageChange} />
      <pre
        className="m-0 overflow-auto font-mono [font-variant-ligatures:none]"
        style={{ fontSize, lineHeight: `${(fontSize * 5) / 3}px`, fontFamily }}
      >
        <code className="block min-w-full w-max">
          {lines.map((tokens, i) => {
            const change = lineChanges[i + 1]
            const value = change === 'removed' ? '−' : String(++number)
            return (
              <span
                key={i}
                data-change={change}
                className="flex min-w-full border-l border-transparent data-[change=added]:border-code-added-indicator
 data-[change=added]:bg-code-added data-[change=removed]:border-code-removed-indicator data-[change=removed]:bg-code-removed"
              >
                <span
                  aria-hidden="true"
                  className={cx(
                    'sticky left-0 w-7 shrink-0 select-none pr-0.5 text-right text-code-gutter',
                    change === 'added' && 'text-code-added-indicator',
                    change === 'removed' && 'text-code-removed-indicator',
                  )}
                >
                  {value}
                </span>
                <span className="block flex-1 whitespace-pre pl-2 pr-4">
                  <Tokens tokens={tokens} />
                  {'\n'}
                </span>
              </span>
            )
          })}
        </code>
      </pre>
    </figure>
  )
}

export interface CodeEditorProps {
  name?: string
  value?: string
  onValueChange?: (value: string) => void
  language: string
  label: string
  theme?: CodeTheme
  fontSize?: number
  fontFamily?: string
  readOnly?: boolean
  invalid?: boolean
  actions?: ReactNode
  showToolbar?: boolean
  className?: string
}

export function CodeEditor({
  name,
  value: suppliedValue = '',
  onValueChange,
  language,
  label,
  theme: suppliedTheme,
  fontSize: suppliedFontSize,
  fontFamily: suppliedFontFamily,
  readOnly,
  invalid,
  actions,
  showToolbar = true,
  className,
}: CodeEditorProps) {
  const saved = useSavedCodeAppearance()
  const theme = suppliedTheme ?? saved.theme
  const fontSize = suppliedFontSize ?? saved.fontSize
  const fontFamily = suppliedFontFamily ?? saved.fontFamily
  const field = useFormField<string>(name)
  const value = field ? (field.value ?? '') : suppliedValue
  const tokens = useTokens(value, language, theme)
  const content = useRef<HTMLPreElement>(null),
    gutter = useRef<HTMLDivElement>(null)
  const style: CSSProperties = { fontSize, lineHeight: `${(fontSize * 5) / 3}px`, fontFamily, tabSize: 2 }
  return (
    <div
      data-slot="code-editor"
      data-form-field={name}
      data-code-theme={theme}
      className={cx(
        'flex min-w-0 w-full flex-col overflow-hidden rounded-md bg-code-background text-code-foreground',
        className,
      )}
    >
      {showToolbar ? <Language language={language} actions={actions} header /> : null}
      <div
        className="relative min-h-32 w-full"
        style={{ height: Math.max(128, ((value.split('\n').length + 1) * (fontSize * 5)) / 3 + 16) }}
      >
        <div
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 overflow-hidden font-mono [font-variant-ligatures:none]"
          style={style}
        >
          <pre ref={content} className="absolute top-2 left-9 m-0 w-max min-w-full font-mono" style={style}>
            <code>
              {tokens.map((line, i) => (
                <span key={i} className="block whitespace-pre">
                  <Tokens tokens={line} />
                  {'\n'}
                </span>
              ))}
            </code>
          </pre>
          <div className="absolute inset-y-0 left-0 w-7 overflow-hidden bg-code-background">
            <div ref={gutter} className="pt-2 pr-0.5 text-right text-code-gutter">
              {tokens.map((_, i) => (
                <div key={i}>{i + 1}</div>
              ))}
            </div>
          </div>
        </div>
        <MultilineField
          name={name}
          aria-label={label}
          aria-invalid={invalid}
          value={value}
          readOnly={readOnly}
          spellCheck={false}
          autoCapitalize="off"
          autoCorrect="off"
          wrap="off"
          className="absolute inset-0 h-full min-h-0 w-full resize-none rounded-none border-0 bg-transparent py-2
pl-9 pr-4 font-mono text-transparent caret-foreground selection:bg-ring/25
[font-variant-ligatures:none] [-webkit-text-fill-color:transparent]"
          style={{ ...style, caretColor: 'var(--code-foreground)' }}
          onChange={(event) => onValueChange?.(event.target.value)}
          onScroll={(event) => {
            const { scrollTop, scrollLeft } = event.currentTarget
            if (content.current) content.current.style.transform = `translate(${-scrollLeft}px, ${-scrollTop}px)`
            if (gutter.current) gutter.current.style.transform = `translateY(${-scrollTop}px)`
          }}
        />
      </div>
    </div>
  )
}
