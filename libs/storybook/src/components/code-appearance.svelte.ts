import { MediaQuery } from 'svelte/reactivity'
import { userPreferenceDefaults } from '@compallm/contract/preference'
import { getCodeAppearance } from '../context'
import { optionalContext } from './form-context.svelte'
export function codeAppearance() {
  const supplied = optionalContext(getCodeAppearance)
  const systemLight = new MediaQuery('(prefers-color-scheme: light)', false)
  return () => {
    if (supplied) return supplied()
    const selected = userPreferenceDefaults.codeAppearance.theme as string
    const mode = selected === 'FOLLOW_INTERFACE' ? (userPreferenceDefaults.theme.mode as string) : selected
    return {
      theme: (mode === 'LINEAR_LIGHT' || mode === 'LIGHT' || (mode === 'SYSTEM' && systemLight.current) ? 'light' : 'dark') as
        'dark' | 'light',
      fontSize: Number(userPreferenceDefaults.codeAppearance.font.size.slice(2)),
      fontFamily:
        (userPreferenceDefaults.codeAppearance.font.family as string) === 'MONOSPACE' ? 'monospace' : 'var(--font-code)',
    }
  }
}
