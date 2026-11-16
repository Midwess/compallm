import { cedarWorker } from '../../workers/cedar-client'

export function projectPolicy(source: string, actionsByName: ReadonlyMap<string, string>, signal?: AbortSignal) {
  return cedarWorker.request('projectPolicy', [source, [...actionsByName]], signal)
}
export function formatPolicySource(source: string, signal?: AbortSignal) {
  return cedarWorker.request('formatPolicySource', [source], signal)
}
export function editPolicyActions(
  source: string,
  actions: string[],
  enabled: boolean,
  actionsByName: ReadonlyMap<string, string>,
  retained: Record<string, string[]>,
  signal?: AbortSignal,
) {
  return cedarWorker.request('editPolicyActions', [source, actions, enabled, [...actionsByName], retained], signal)
}

export function preparePolicySource(source: string, current: string, saved: string, signal?: AbortSignal) {
  return cedarWorker.request('preparePolicySource', [source, current, saved], signal)
}
