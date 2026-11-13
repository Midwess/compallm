import {
  confirmedPreferenceDraft,
  parsePreferencePatch,
  preferenceSelection,
  type SettingsPageControllers,
  type SettingsState,
} from '@compallm/storybook/pages/settings'
import type { MutationGenqlSelection } from '@compallm/contract/graphql'
import { graphql } from '../../graphql-client'

type Contract = SettingsPageControllers['preferences']
export class PreferencesController implements Contract {
  updatePreferences = async (patch: Partial<SettingsState['preferences']>, signal: AbortSignal) => {
    const parsed = parsePreferencePatch(patch)
    if (!parsed.input) throw new Error('Invalid preference patch')
    if (!Object.keys(parsed.input).length) return patch
    const selection = {
      userPreferencesUpdate: { __args: { input: parsed.input }, node: preferenceSelection },
    } satisfies MutationGenqlSelection
    const {
      userPreferencesUpdate: { node },
    } = await graphql(signal).mutation(selection)
    return confirmedPreferenceDraft(node, patch)
  }
}
