import type { CurrentUserResponse } from '../contract/api'

export function organizationHome(user: Pick<CurrentUserResponse, 'organizationName'>) {
  return `/${encodeURIComponent(user.organizationName)}/chat`
}
