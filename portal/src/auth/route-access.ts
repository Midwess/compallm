import { redirect, type LoaderFunctionArgs } from 'react-router'
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
  const user = await checkSession(true)
  if (user) return redirect(organizationHome(user))
  return null
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
