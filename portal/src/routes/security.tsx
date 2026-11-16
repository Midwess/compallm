import { useEffect } from 'react'
import { redirect } from 'react-router'
import { SecurityEnhancementPage } from '@compallm/storybook'
import { authController } from '../auth/controller'
import { passkeyController } from '../passkey/instance'
import { urlController } from '../url/controller'
import { organizationRoot } from '../organization/paths'
export async function loader() {
  const user = await authController.ensureCurrentUser()
  if (!user) return redirect('/login')
  if (authController.securityOfferSession !== user.id) return redirect(organizationRoot(user))
  authController.securityOfferSession = undefined
  return null
}
function next() {
  const user = authController.store.getState().currentUser
  authController.securityOfferSession = undefined
  if (user) urlController.navigate(organizationRoot(user), { replace: true })
}
export function Component() {
  useEffect(() => {
    const close = passkeyController.startView()
    const stop = passkeyController.store.subscribe((state, before) => {
      if (before.dialog?.kind === 'add' && !state.dialog && state.records !== before.records) next()
    })
    return () => {
      stop()
      close()
    }
  }, [])
  return <SecurityEnhancementPage onSkip={next} />
}
