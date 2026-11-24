import { HoudiniClient, type ClientPlugin } from './generated/runtime/client'
import { fetch as network, type RequestHandler } from './generated/runtime/plugins/fetch'

export interface TransportOperation {
  kind: 'query' | 'mutation' | 'other'
  reusable: boolean
  fetch?: typeof globalThis.fetch
}
export type TransportHandler = (
  args: Parameters<RequestHandler>[0],
  request: AbortController,
  operation?: TransportOperation,
) => ReturnType<RequestHandler>
function sensitive(selection: { fields?: Record<string, { keyRaw: string; selection?: unknown }> } | undefined): boolean {
  return Object.values(selection?.fields ?? {}).some(
    (field) =>
      ['session', 'sessions', 'invitationContext', 'passkeys', 'cedarSource', 'cedarEntities'].includes(
        field.keyRaw.split('(')[0],
      ) || sensitive(field.selection as Parameters<typeof sensitive>[0]),
  )
}
let handler: TransportHandler | undefined
export function configureTransport(next: TransportHandler) {
  handler = next
  return () => {
    if (handler === next) handler = undefined
  }
}
const transport: ClientPlugin = () => ({
  network(context, handlers) {
    const current = handler
    if (!current) return handlers.next(context)
    const kind =
      context.artifact.kind === 'HoudiniQuery' ? 'query' : context.artifact.kind === 'HoudiniMutation' ? 'mutation' : 'other'
    const operation: TransportOperation = {
      kind,
      reusable: kind === 'query' && !/^(Auth|Passkey)/.test(context.artifact.name) && !sensitive(context.artifact.selection),
      fetch: context.fetch,
    }
    const hooks = network((args) => current(args, context.abortController, operation))()
    if (hooks && !Array.isArray(hooks) && typeof hooks !== 'function') return hooks.network?.(context, handlers)
  },
})
export default new HoudiniClient({ fetchParams: () => ({ credentials: 'same-origin' }), plugins: [transport] })
