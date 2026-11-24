import type { Handle } from '@sveltejs/kit/hooks'
import { PortalRequest } from './lib/server/portal'

export const handle: Handle = async ({ event, resolve }) => {
  event.locals.portal = new PortalRequest(event)
  const response = await resolve(event, {
    transformPageChunk: ({ html }) => {
      const appearance = event.locals.appearance
      return appearance
        ? html.replace(
            '<html lang="en" data-theme="basic-dark" class="dark">',
            `<html lang="en" data-theme="${appearance.theme}" data-appearance="${appearance.dark ? 'dark' : 'light'}" data-text-size="${appearance.size}" data-underline-links="${appearance.underline}" class="${appearance.dark ? 'dark' : ''}">`,
          )
        : html
    },
  })
  response.headers.set('Cache-Control', 'private, no-store')
  response.headers.set('Referrer-Policy', 'strict-origin-when-cross-origin')
  if (!['/', '/signup', '/login', '/signin'].includes(event.url.pathname))
    response.headers.set('X-Robots-Tag', 'noindex, nofollow')
  return response
}
