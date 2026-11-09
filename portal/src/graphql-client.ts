import { createClient } from '@compallm/contract/graphql'

/** A safe GraphQL or HTTP failure. Partial results never establish authenticated state. */
export class ApiError extends Error {
  readonly status: number
  readonly code: string
  readonly info: { message: string }

  constructor(message: string, status: number, code: string) {
    super(message)
    this.status = status
    this.code = code
    this.info = { message }
  }
}

const requestFetch: typeof fetch = async (input, init) => {
  const response = await fetch(input, init)
  const payload = await response
    .clone()
    .json()
    .catch(() => null)
  const failure = payload?.errors?.[0]
  if (!response.ok || failure) {
    const message = typeof failure?.message === 'string' ? failure.message : 'Request failed. Please try again.'
    const status = response.ok && Number.isInteger(failure?.extensions?.status) ? failure.extensions.status : response.status
    throw new ApiError(
      message,
      status,
      typeof failure?.extensions?.code === 'string' ? failure.extensions.code : 'REQUEST_FAILED',
    )
  }
  return response
}

/** Consumer-owned configuration; each call can cancel its own request. */
export const graphql = (signal?: AbortSignal) =>
  createClient({
    url: '/api/graphql',
    credentials: 'same-origin',
    signal,
    fetch: requestFetch,
  })
