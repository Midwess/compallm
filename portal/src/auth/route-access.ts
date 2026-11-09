import { redirect, type LoaderFunctionArgs } from 'react-router'
import { emailRequestSchema } from '@compallm/contract/auth'
import { authController } from './controller'
import { notify } from '../notifications/toast'
import { organizationHome } from '../organization/paths'

async function checkSession(refresh = false) {
  try {
    return await (refresh ? authController.refreshCurrentUser(undefined, false) : authController.ensureCurrentUser())
  } catch (error) {
    notify.error(error)
    throw error
  }
}

export async function guestOnly() {
  // A user-only identity check would clear an in-progress OTP cookie.
  if (authController.store.getState().step === 'code') return null
  const user = await checkSession(true)
  return user ? redirect(organizationHome(user)) : null
}

export async function requireCurrentUser({ params }: LoaderFunctionArgs) {
  const user = await checkSession()
  if (!user) return redirect('/login')
  if (params.organizationName !== user.organizationName) return redirect(organizationHome(user))
  return null
}

export async function organizationIndex() {
  const user = await checkSession()
  return redirect(user ? organizationHome(user) : '/login')
}

export function confirmationAccess({ request }: LoaderFunctionArgs) {
  const url = new URL(request.url)
  const user = authController.store.getState().currentUser
  if (user) return redirect(organizationHome(user))
  const email = url.searchParams.get('email')
  const mode = url.pathname.startsWith('/signup') ? 'signup' : 'signin'
  if (!emailRequestSchema(mode).safeParse({ email, delivery: 'COOKIE' }).success) {
    return redirect(url.pathname.startsWith('/signup') ? '/signup' : '/login')
  }
  return null
}
