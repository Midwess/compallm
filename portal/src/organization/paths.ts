import type { AuthUser } from '@compallm/storybook'

export function organizationHome(user: Pick<AuthUser, 'organizationName'>) {
  return `/${encodeURIComponent(user.organizationName)}/chat`
}

export function organizationRoot(user: Pick<AuthUser, 'organizationName'>) {
  return `/${encodeURIComponent(user.organizationName)}`
}
