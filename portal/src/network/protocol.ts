import type { NetworkRequest, NetworkResult, SerializedNetworkError } from './service'

export const networkProtocol = 'compallm-network-v1'
export type WorkerRequest =
  | { protocol: typeof networkProtocol; type: 'ping' }
  | { protocol: typeof networkProtocol; type: 'reset'; client: string; epoch: number }
  | { protocol: typeof networkProtocol; type: 'request'; client: string; epoch: number; request: NetworkRequest }
export type WorkerReply =
  | { type: 'ready'; protocol: typeof networkProtocol }
  | { type: 'started' }
  | { type: 'result'; result: NetworkResult }
  | { type: 'error'; error: SerializedNetworkError }
export type WorkerCancellation = { type: 'cancel' }
