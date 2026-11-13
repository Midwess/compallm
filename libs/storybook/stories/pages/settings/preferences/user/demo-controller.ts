import merge from 'lodash-es/merge.js'
import { userPreferenceDefaults } from '@compallm/contract/preference'
import {
  createUserPreferenceStore,
  loadedPreferenceDraft,
  parsePreferencePatch,
  type UserPreferenceController,
  type UserPreferenceValues,
  type PreferenceDraft,
} from '../../../../../src/pages/settings'

const defaults: UserPreferenceValues = {
  ...userPreferenceDefaults,
  sidebar: {
    ...userPreferenceDefaults.sidebar,
    entries: userPreferenceDefaults.sidebar.entries.map((entry) => ({ ...entry })),
  },
}

export class DemoUserPreferenceController implements UserPreferenceController {
  readonly store = createUserPreferenceStore({
    saved: structuredClone(defaults),
    draft: loadedPreferenceDraft(structuredClone(defaults)),
    status: 'ready',
  })
  constructor(private readonly failWrites: boolean | 'conflict' = false) {}
  load = async () => {
    this.store.form.reset({ draft: loadedPreferenceDraft(this.store.getState().saved!) })
    this.store.setState({ status: 'ready' })
  }
  cancel = () => this.store.form.stop()
  update = async (patch: Partial<PreferenceDraft>, signal: AbortSignal) => {
    if (signal.aborted) throw new DOMException('Cancelled', 'AbortError')
    if (this.failWrites) throw new Error('Could not save preferences. Your draft is retained.')
    const saved = this.store.getState().saved!
    const draft = { ...loadedPreferenceDraft(saved), ...patch }
    const { sidebar, ...groups } = parsePreferencePatch(draft).input!
    const next = merge({}, saved, groups) as UserPreferenceValues
    next.sidebar = {
      badgeStyle: sidebar?.badgeStyle ?? saved.sidebar.badgeStyle,
      entries: draft.sidebarOrder.map((item) => ({
        item,
        visibility: draft[
          `sidebar${item}` as keyof PreferenceDraft
        ] as UserPreferenceValues['sidebar']['entries'][number]['visibility'],
      })),
    }
    this.store.setState({ saved: next })
    return patch
  }
}
