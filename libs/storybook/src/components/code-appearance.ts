import { useContext, useSyncExternalStore } from 'react'
import { useStore } from 'zustand'
import { createStore } from 'zustand/vanilla'
import { userPreferenceDefaults } from '@compallm/contract/preference'
import type {
  PreferenceAppearanceMode,
  PreferenceCodeTheme,
  PreferenceFontFamily,
  PreferencePixelFontSize,
} from '@compallm/contract/graphql'
import { UserPreferenceControllerContext } from '../context'

const empty = createStore(() => ({ saved: null }))
const query = '(prefers-color-scheme: light)'
const subscribe = (notify: () => void) => {
  const media = window.matchMedia(query)
  media.addEventListener('change', notify)
  return () => media.removeEventListener('change', notify)
}

export function useSystemLight() {
  return useSyncExternalStore(
    subscribe,
    () => window.matchMedia(query).matches,
    () => false,
  )
}

export function resolveCodeAppearance(
  selected: PreferenceCodeTheme,
  interfaceMode: PreferenceAppearanceMode,
  font: PreferenceFontFamily,
  size: PreferencePixelFontSize,
  systemLight: boolean,
) {
  const mode = selected === 'FOLLOW_INTERFACE' ? interfaceMode : selected
  return {
    theme: (mode === 'LINEAR_LIGHT' || mode === 'LIGHT' || (mode === 'SYSTEM' && systemLight) ? 'light' : 'dark') as
      'light' | 'dark',
    fontSize: Number(size.slice(2)),
    fontFamily: font === 'MONOSPACE' ? 'monospace' : 'var(--font-code)',
  }
}

export function useSavedCodeAppearance() {
  const controller = useContext(UserPreferenceControllerContext)
  const store = controller?.store ?? empty
  const selected = useStore(store, (state) => state.saved?.codeAppearance.theme ?? userPreferenceDefaults.codeAppearance.theme)
  const mode = useStore(store, (state) => state.saved?.theme.mode ?? userPreferenceDefaults.theme.mode)
  const font = useStore(
    store,
    (state) => state.saved?.codeAppearance.font.family ?? userPreferenceDefaults.codeAppearance.font.family,
  )
  const size = useStore(
    store,
    (state) => state.saved?.codeAppearance.font.size ?? userPreferenceDefaults.codeAppearance.font.size,
  )
  return resolveCodeAppearance(selected, mode, font, size, useSystemLight())
}
