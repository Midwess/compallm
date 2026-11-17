import type { PasskeyOptions, PasskeyFinishInput } from '@compallm/contract/graphql'
type Options = Omit<PasskeyOptions, '__typename' | 'credentials' | 'extensions'> & {
  extensions: Omit<PasskeyOptions['extensions'], '__typename'>
  credentials: Pick<PasskeyOptions['credentials'][number], 'id' | 'transports'>[]
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
    throw new Error('Passkeys require a supported browser and a secure connection. You can use email instead.')
  const descriptors = options.credentials.map((c) => ({
    type: 'public-key' as const,
    id: decode(c.id),
    transports: c.transports as AuthenticatorTransport[],
  }))
  const result = registration
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
  if (!(result instanceof PublicKeyCredential)) throw new Error('Passkey verification was not completed.')
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
export const ceremonySelection = {
  token: true,
  expiresAt: true,
  options: {
    challenge: true,
    rpId: true,
    rpName: true,
    userId: true,
    userName: true,
    userDisplayName: true,
    algorithms: true,
    credentials: { id: true, transports: true },
    timeout: true,
    userVerification: true,
    residentKey: true,
    extensions: { credProps: true, credentialProtectionPolicy: true, enforceCredentialProtectionPolicy: true },
  },
} as const
export const passkeySelection = { credentialId: true, id: true, name: true, registeredAt: true, lastUsedAt: true } as const
