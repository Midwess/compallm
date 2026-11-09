import type { CurrentUserResponse } from '@compallm/contract/preview'

export function organizationHome(user: Pick<CurrentUserResponse, 'organizationName'>) {
  return `/${encodeURIComponent(user.organizationName)}/chat`
}
