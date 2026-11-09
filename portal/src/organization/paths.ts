import type { CurrentUserResponse } from '@compallm/contract/types'

export function organizationHome(user: Pick<CurrentUserResponse, 'organizationName'>) {
  return `/${encodeURIComponent(user.organizationName)}/chat`
}
