import type { AuthPasskeyAuthenticationStart2$result } from '@compallm/contract/houdini'
import { credential as browserCredential } from '../passkey/browser'
type Options = AuthPasskeyAuthenticationStart2$result['passkeyAuthenticationStart']['node']['options']
export { PasskeyError, supportsPasskeys, decode, encode } from '../passkey/browser'
export function credential(options: Options, token: string, signal: AbortSignal) {
  return browserCredential(options, token, false, signal)
}
