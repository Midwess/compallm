import { createHighlighterCore, type HighlighterCore, type ThemedToken } from 'shiki/core'
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript'
import cedarGrammar from '../assets/cedar/cedar.tmLanguage.json'

export const codeThemes = {
  dark: {
    name: 'compallm-dark',
    type: 'dark' as const,
    colors: { 'editor.background': '#212123', 'editor.foreground': '#e4e5e7' },
    tokenColors: [
      { scope: ['comment', 'punctuation.definition.comment'], settings: { foreground: '#5c5d5f' } },
      { scope: ['keyword', 'storage'], settings: { foreground: '#f79ce0' } },
      { scope: ['keyword.control.flow', 'keyword.control.return'], settings: { foreground: '#ca9cfc' } },
      { scope: ['string'], settings: { foreground: '#ffdf9f' } },
      { scope: ['variable', 'constant.language'], settings: { foreground: '#8fa6ff' } },
      {
        scope: ['variable.other.property', 'variable.parameter', 'support.variable.property', 'meta.object-literal.key'],
        settings: { foreground: '#f7bf8b' },
      },
      { scope: ['constant.numeric', 'entity.name.function', 'support.function'], settings: { foreground: '#83dcdc' } },
      { scope: ['entity.name.type', 'support.type', 'support.class'], settings: { foreground: '#76cff6' } },
    ],
  },
  light: {
    name: 'compallm-light',
    type: 'light' as const,
    colors: { 'editor.background': '#f5f5f6', 'editor.foreground': '#25262a' },
    tokenColors: [
      { scope: ['comment'], settings: { foreground: '#76777c' } },
      { scope: ['keyword', 'storage'], settings: { foreground: '#a52b7e' } },
      { scope: ['string'], settings: { foreground: '#805312' } },
      { scope: ['variable', 'constant.language'], settings: { foreground: '#4056ad' } },
      { scope: ['variable.other.property', 'variable.parameter'], settings: { foreground: '#a0562c' } },
      { scope: ['constant.numeric', 'entity.name.function', 'support.function'], settings: { foreground: '#217d7d' } },
      { scope: ['entity.name.type', 'support.type'], settings: { foreground: '#286f9c' } },
    ],
  },
}

let engine: Promise<HighlighterCore> | undefined
const languages = new Map<string, Promise<void>>()
const loaders = {
  typescript: () => import('shiki/langs/typescript.mjs'),
  javascript: () => import('shiki/langs/javascript.mjs'),
  json: () => import('shiki/langs/json.mjs'),
  bash: () => import('shiki/langs/bash.mjs'),
}

export async function highlightCode(code: string, language: string, theme: 'dark' | 'light'): Promise<ThemedToken[][]> {
  const aliases: Record<string, string> = { ts: 'typescript', js: 'javascript', shell: 'bash' }
  const name = aliases[language.toLowerCase()] ?? language.toLowerCase()
  if (!(name in loaders) && name !== 'cedar') return code.split('\n').map((content) => [{ content, offset: 0 }])
  engine ??= createHighlighterCore({
    themes: [codeThemes.dark, codeThemes.light],
    langs: [],
    engine: createJavaScriptRegexEngine(),
  })
  const highlighter = await engine
  if (!languages.has(name))
    languages.set(
      name,
      (async () => {
        if (name === 'cedar') await highlighter.loadLanguage({ ...cedarGrammar, name: 'cedar', aliases: ['Cedar'] })
        else await highlighter.loadLanguage((await loaders[name as keyof typeof loaders]()).default)
      })(),
    )
  await languages.get(name)
  return highlighter.codeToTokens(code, { lang: name, theme: codeThemes[theme].name }).tokens
}
