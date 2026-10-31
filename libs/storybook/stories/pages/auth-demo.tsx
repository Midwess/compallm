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
  const [controller] = useState(() => new DemoAuthController(mode, scenario, initialState))
  useEffect(() => controller.start(), [controller])
  const Page = mode === 'signup' ? SignupPage : SigninPage
  return <Page controller={controller} />
}
