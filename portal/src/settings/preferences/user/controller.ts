import {
  createUserPreferenceStore,
  loadedPreferenceDraft,
  confirmedPreferenceDraft,
  parsePreferencePatch,
  preferenceSelection,
  type UserPreferenceController as Contract,
  type PreferenceDraft,
} from '@compallm/storybook/pages/settings'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { ApiError, graphql } from '../../../graphql-client'
import { notify } from '../../../notifications/toast'

export class UserPreferenceController implements Contract {
  readonly store = createUserPreferenceStore()
  private scope = new AbortController()
  private read?: AbortController

  cancel = () => {
    this.scope.abort()
    this.read?.abort()
    this.read = undefined
    this.store.form.stop()
    this.store.setState({ status: this.store.getState().saved ? 'ready' : 'idle' })
  }

  load = async () => {
    if (this.scope.signal.aborted) this.scope = new AbortController()
    if (this.store.form.dirty || this.store.form.pending) return
    const scope = this.scope
    this.read?.abort()
    const request = new AbortController()
    this.read = request
    this.store.setState({ status: 'loading' })
    try {
      const selection = { userPreferences: { node: preferenceSelection } } satisfies QueryGenqlSelection
      const {
        userPreferences: { node },
      } = await graphql(AbortSignal.any([scope.signal, request.signal])).query(selection)
      if (scope.signal.aborted || this.read !== request) return
      const draft = loadedPreferenceDraft(node, this.store.form.values.draft!.codeLanguage)
      this.store.form.reset({ draft })
      this.store.setState({ saved: node, status: 'ready' })
    } catch (error) {
      if (scope.signal.aborted || this.read !== request) return
      this.store.setState({ status: error instanceof ApiError && [401, 403].includes(error.status) ? 'denied' : 'error' })
      notify.apiError(error, () => {
        if (!scope.signal.aborted && this.read === request) void this.load()
      })
    }
  }

  update = async (patch: Partial<PreferenceDraft>, signal: AbortSignal) => {
    const parsed = parsePreferencePatch(patch)
    if (!parsed.input) throw new Error('Invalid preference patch')
    if (!Object.keys(parsed.input).length) return patch
    const scope = this.scope
    if (scope.signal.aborted || this.store.getState().status !== 'ready') throw new DOMException('Cancelled', 'AbortError')
    this.read?.abort()
    this.read = undefined
    const selection = {
      userPreferencesUpdate: { __args: { input: parsed.input }, node: preferenceSelection },
    } satisfies MutationGenqlSelection
    const {
      userPreferencesUpdate: { node },
    } = await graphql(AbortSignal.any([signal, scope.signal])).mutation(selection)
    if (scope.signal.aborted || signal.aborted || this.scope !== scope) throw new DOMException('Cancelled', 'AbortError')
    this.store.setState({ saved: node })
    return confirmedPreferenceDraft(node, patch)
  }
}
