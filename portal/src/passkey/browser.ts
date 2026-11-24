import type { PasskeyFinishInput } from '@compallm/contract/houdini'
type PasskeyOptions =
  import('@compallm/contract/houdini').PasskeyPasskeyRegistrationStart$result['passkeyRegistrationStart']['node']['options']
type Options = Omit<PasskeyOptions, '__typename' | 'credentials' | 'extensions'> & {
  extensions: Omit<PasskeyOptions['extensions'], '__typename'>
  credentials: Pick<PasskeyOptions['credentials'][number], 'id' | 'transports'>[]
}
export class PasskeyError extends Error {
  get info() {
    return { message: this.message }
  }
}
export function supportsPasskeys() {
  return globalThis.isSecureContext && typeof PublicKeyCredential !== 'undefined' && Boolean(navigator.credentials)
}
export function decode(value: string): ArrayBuffer {
  const binary = atob(value.replace(/-/g, '+').replace(/_/g, '/'))
  return Uint8Array.from(binary, (c) => c.charCodeAt(0)).buffer
}
export function encode(value: ArrayBuffer): string {
  return btoa(String.fromCharCode(...new Uint8Array(value)))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=+$/, '')
}
export async function credential(
  options: Options,
  token: string,
  registration: boolean,
  signal: AbortSignal,
): Promise<PasskeyFinishInput> {
  if (!supportsPasskeys())
    throw new PasskeyError('Passkeys require a supported browser and a secure connection. You can use email instead.')
  const hostname = window.location.hostname
  if (hostname !== options.rpId && !hostname.endsWith(`.${options.rpId}`)) {
    const address = new URL(window.location.origin)
    address.hostname = options.rpId
    throw new PasskeyError(
      `Passkeys are configured for ${options.rpId}. Open ${address.origin} to use your passkey, or sign in with email here.`,
    )
  }
  const descriptors = options.credentials.map((c) => ({
    type: 'public-key' as const,
    id: decode(c.id),
    transports: c.transports as AuthenticatorTransport[],
  }))
  let result: Credential | null
  try {
    result = registration
      ? await navigator.credentials.create({
          signal,
          publicKey: {
            challenge: decode(options.challenge),
            rp: { id: options.rpId, name: options.rpName ?? 'Compallm' },
            user: {
              id: decode(options.userId!),
              name: options.userName!,
              displayName: options.userDisplayName || options.userName!,
            },
            pubKeyCredParams: options.algorithms.map((alg) => ({ type: 'public-key', alg })),
            excludeCredentials: descriptors,
            authenticatorSelection: { userVerification: 'required', residentKey: 'required', requireResidentKey: true },
            attestation: 'none',
            extensions: {
              ...(options.extensions.credProps !== null ? { credProps: options.extensions.credProps } : {}),
              ...(options.extensions.credentialProtectionPolicy
                ? { credentialProtectionPolicy: options.extensions.credentialProtectionPolicy }
                : {}),
              ...(options.extensions.enforceCredentialProtectionPolicy !== null
                ? { enforceCredentialProtectionPolicy: options.extensions.enforceCredentialProtectionPolicy }
                : {}),
            },
            timeout: options.timeout,
          },
        })
      : await navigator.credentials.get({
          signal,
          publicKey: {
            challenge: decode(options.challenge),
            rpId: options.rpId,
            ...(descriptors.length ? { allowCredentials: descriptors } : {}),
            userVerification: 'required',
            timeout: options.timeout,
          },
        })
  } catch (error) {
    if (error instanceof DOMException && error.name === 'NotSupportedError')
      throw new PasskeyError(
        'This browser or device does not support the required passkey verification. Use a supported browser or sign in with email.',
      )
    if (error instanceof DOMException && error.name === 'SecurityError')
      throw new PasskeyError(
        `Passkey verification requires the configured ${options.rpId} address and a secure connection. You can sign in with email instead.`,
      )
    throw error
  }
  if (!(result instanceof PublicKeyCredential)) throw new PasskeyError('Passkey verification was not completed.')
  const response = result.response
  const common = {
    token,
    id: result.id,
    rawId: encode(result.rawId),
    clientDataJson: encode(response.clientDataJSON),
    delivery: 'COOKIE' as const,
  }
  if (response instanceof AuthenticatorAttestationResponse)
    return { ...common, attestationObject: encode(response.attestationObject), transports: response.getTransports?.() ?? [] }
  if (response instanceof AuthenticatorAssertionResponse)
    return {
      ...common,
      authenticatorData: encode(response.authenticatorData),
      signature: encode(response.signature),
      userHandle: response.userHandle ? encode(response.userHandle) : null,
    }
  throw new Error('The browser returned an unsupported credential.')
}
