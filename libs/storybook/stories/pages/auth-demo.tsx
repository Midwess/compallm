import { AuthControllerContext } from '../../src/context'
import { useEffect, useState } from 'react'
import { SignupPage } from '../../src/pages/signup'
import { SigninPage } from '../../src/pages/signin'
import type { AuthInitialState } from '../../src/pages/auth/types'
import { DemoAuthController, type AuthDemoScenario } from './demo-auth-controller'

export type { AuthDemoScenario } from './demo-auth-controller'

export function AuthDemo({
  mode,
  scenario = 'default',
  initialState,
}: {
  mode: 'signup' | 'signin'
  scenario?: AuthDemoScenario
  initialState?: AuthInitialState
}) {
  const [controller] = useState(() => {
    const result = new DemoAuthController(mode, scenario, initialState)
    if (scenario === 'pending') result.store.setState({ pending: initialState?.step === 'code' ? 'verify' : 'request' })
    return result
  })
  useEffect(() => controller.start(), [controller])
  const Page = mode === 'signup' ? SignupPage : SigninPage
  return (
    <AuthControllerContext value={controller}>
      <Page />
    </AuthControllerContext>
  )
}
