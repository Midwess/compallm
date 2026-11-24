import type { ThemedToken } from 'shiki/core'
export function codeTokens(source: () => string, language: () => string, theme: () => 'dark' | 'light'): () => ThemedToken[][] {
  let loaded = $state<{ code: string; language: string; theme: 'dark' | 'light'; lines: ThemedToken[][] }>()
  $effect(() => {
    const code = source(),
      lang = language(),
      selected = theme()
    let active = true
    const timer = setTimeout(() => {
      void import('./code-highlighter')
        .then((module) => module.highlightCode(code, lang, selected))
        .then((lines) => {
          if (active) loaded = { code, language: lang, theme: selected, lines }
        })
        .catch(() => {})
    }, 80)
    return () => {
      active = false
      clearTimeout(timer)
    }
  })
  return () =>
    loaded?.code === source() && loaded.language === language() && loaded.theme === theme()
      ? loaded.lines
      : source()
          .split('\n')
          .map((content) => [{ content, offset: 0 }])
}
