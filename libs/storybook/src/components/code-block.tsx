import { useEffect, useRef, useState, type CSSProperties, type ReactNode } from 'react'
import type { ThemedToken } from 'shiki/core'
import { MultilineField } from './fields'
import { Select } from './dropdowns'
import { cx } from './shared'

type CodeTheme = 'dark' | 'light'
type Change = 'added' | 'removed'
const labels: Record<string, string> = {
  typescript: 'TypeScript',
  javascript: 'JavaScript',
  json: 'JSON',
  bash: 'Bash',
  cedar: 'Cedar',
}
const background = { dark: '#212123', light: '#f5f5f6' }
const foreground = { dark: '#e4e5e7', light: '#25262a' }

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
}: {
  language: string
  onChange?: (language: string) => void
  actions?: ReactNode
}) {
  const name = labels[language.toLowerCase()] ?? language
  return (
    <div className="absolute top-2 right-5 z-10 flex items-center gap-2 text-xs">
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
        <span className="pointer-events-none rounded-lg bg-elevated px-2.5 py-1 font-sans text-xs leading-4 text-text">
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
  theme = 'dark',
  fontSize = 12,
  fontFamily,
  lineChanges = {},
  onLanguageChange,
  className,
}: CodePreviewProps) {
  const lines = useTokens(code, language, theme)
  let number = 0
  return (
    <figure
      data-slot="code-block"
      className={cx('relative m-0 overflow-hidden rounded-md', className)}
      style={{ backgroundColor: background[theme], color: foreground[theme] }}
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
            const added = theme === 'dark' ? '#212d26' : '#e9f3e9',
              removed = theme === 'dark' ? '#362628' : '#f7e9e9'
            return (
              <span
                key={i}
                data-change={change}
                className="flex min-w-full border-l border-transparent"
                style={{
                  backgroundColor: change === 'added' ? added : change === 'removed' ? removed : undefined,
                  borderLeftColor: change === 'added' ? '#588b5d' : change === 'removed' ? '#975f63' : undefined,
                }}
              >
                <span
                  aria-hidden="true"
                  className="sticky left-0 w-11 shrink-0 select-none pr-0.5 text-right"
                  style={{ color: change === 'added' ? '#588b5d' : change === 'removed' ? '#975f63' : '#5c5d5f' }}
                >
                  {value}
                </span>
                <span className="block flex-1 whitespace-pre px-4">
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
  value: string
  onValueChange: (value: string) => void
  onCommit?: () => void
  language: string
  label: string
  theme?: CodeTheme
  fontSize?: number
  fontFamily?: string
  readOnly?: boolean
  invalid?: boolean
  actions?: ReactNode
  className?: string
}

export function CodeEditor({
  value,
  onValueChange,
  onCommit,
  language,
  label,
  theme = 'dark',
  fontSize = 12,
  fontFamily,
  readOnly,
  invalid,
  actions,
  className,
}: CodeEditorProps) {
  const tokens = useTokens(value, language, theme)
  const content = useRef<HTMLPreElement>(null),
    gutter = useRef<HTMLDivElement>(null)
  const style: CSSProperties = { fontSize, lineHeight: `${(fontSize * 5) / 3}px`, fontFamily, tabSize: 2 }
  return (
    <div
      data-slot="code-editor"
      className={cx('relative min-h-32 min-w-0 flex-1 overflow-hidden rounded-md', className)}
      style={{ backgroundColor: background[theme], color: foreground[theme] }}
    >
      <Language language={language} actions={actions} />
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0 overflow-hidden font-mono [font-variant-ligatures:none]"
        style={style}
      >
        <pre ref={content} className="absolute top-0 left-15.5 m-0 w-max min-w-full font-mono" style={style}>
          <code>
            {tokens.map((line, i) => (
              <span key={i} className="block whitespace-pre">
                <Tokens tokens={line} />
                {'\n'}
              </span>
            ))}
          </code>
        </pre>
        <div className="absolute inset-y-0 left-0 w-11 overflow-hidden" style={{ backgroundColor: background[theme] }}>
          <div ref={gutter} className="pr-0.5 text-right" style={{ color: '#5c5d5f' }}>
            {tokens.map((_, i) => (
              <div key={i}>{i + 1}</div>
            ))}
          </div>
        </div>
      </div>
      <MultilineField
        aria-label={label}
        aria-invalid={invalid}
        value={value}
        readOnly={readOnly}
        spellCheck={false}
        autoCapitalize="off"
        autoCorrect="off"
        wrap="off"
        className="absolute inset-0 h-full min-h-0 w-full resize-none rounded-none border-0 bg-transparent p-0
pl-15.5 pr-4 font-mono text-transparent caret-text selection:bg-focus/25
[font-variant-ligatures:none] [-webkit-text-fill-color:transparent]"
        style={{ ...style, caretColor: foreground[theme] }}
        onChange={(event) => onValueChange(event.target.value)}
        onBlur={onCommit}
        onScroll={(event) => {
          const { scrollTop, scrollLeft } = event.currentTarget
          if (content.current) content.current.style.transform = `translate(${-scrollLeft}px, ${-scrollTop}px)`
          if (gutter.current) gutter.current.style.transform = `translateY(${-scrollTop}px)`
        }}
      />
    </div>
  )
}
