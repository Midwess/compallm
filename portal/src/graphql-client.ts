import { createClient, type ValidationIssue } from '@compallm/contract/graphql'
import { beginRequest } from './request-activity'

let invalidAuthorizationHandler: (() => void) | undefined

export function onInvalidAuthorization(handler: () => void) {
  invalidAuthorizationHandler = handler
}

/** A safe GraphQL or HTTP failure. Partial results never establish authenticated state. */
export class ApiError extends Error {
  readonly status: number
  readonly code: string
  readonly validation: ValidationIssue[]
  readonly info: { message: string }

  constructor(message: string, status: number, code: string, validation: ValidationIssue[] = []) {
    super(message)
    this.validation = validation
    this.status = status
    this.code = code
    this.info = { message }
  }
}

type GraphqlFetcher = NonNullable<NonNullable<Parameters<typeof createClient>[0]>['fetcher']>

const request = async (operation: Parameters<GraphqlFetcher>[0], signal?: AbortSignal) => {
  const finish = beginRequest()
  try {
    const response = await fetch('/api/graphql', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      signal,
      body: JSON.stringify(operation),
    })
    let payload
    try {
      payload = await response.json()
    } catch (error) {
      if (signal?.aborted) throw error
      throw new ApiError('Request failed. Please try again.', response.ok ? 502 : response.status, 'INVALID_RESPONSE')
    }
    const failure = payload?.errors?.[0]
    if (!response.ok || failure) {
      if (
        !signal?.aborted &&
        payload?.errors?.some(
          (error: { extensions?: { code?: string } }) => error.extensions?.code === 'INVALID_AUTHORIZATION_STATE',
        )
      ) {
        invalidAuthorizationHandler?.()
      }
      const message = typeof failure?.message === 'string' ? failure.message : 'Request failed. Please try again.'
      const status = response.ok && Number.isInteger(failure?.extensions?.status) ? failure.extensions.status : response.status
      throw new ApiError(
        message,
        status,
        typeof failure?.extensions?.code === 'string' ? failure.extensions.code : 'REQUEST_FAILED',
        Array.isArray(failure?.extensions?.validation)
          ? failure.extensions.validation.filter(
              (issue: ValidationIssue) =>
                issue &&
                Array.isArray(issue.path) &&
                issue.path.every((part) => typeof part === 'string') &&
                typeof issue.message === 'string' &&
                typeof issue.code === 'string',
            )
          : [],
      )
    }
    if (!payload || typeof payload !== 'object' || Array.isArray(payload) || !Object.hasOwn(payload, 'data'))
      throw new ApiError('Request failed. Please try again.', 502, 'INVALID_RESPONSE')
    return payload
  } finally {
    finish()
  }
}

/** Consumer-owned configuration; each call can cancel its own request. */
export const graphql = (signal?: AbortSignal) =>
  createClient({
    fetcher: (operation) => request(operation, signal),
  })
