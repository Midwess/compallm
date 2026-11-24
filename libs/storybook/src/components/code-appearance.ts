import type {
  PreferenceAppearanceMode$options as PreferenceAppearanceMode,
  PreferenceCodeTheme$options as PreferenceCodeTheme,
  PreferenceFontFamily$options as PreferenceFontFamily,
  PreferencePixelFontSize$options as PreferencePixelFontSize,
} from '@compallm/contract/houdini'
export { systemLight as useSystemLight } from '../pages/page-state.svelte'
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
