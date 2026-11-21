import { PasskeyControllerContext } from '@compallm/storybook/context'
import { PasskeyDialogs } from '@compallm/storybook'
import { passkeyController } from './passkey/instance'
import { StrictMode, useEffect } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router/dom'
import { Toaster } from 'sonner'
import { router } from './App.tsx'
import '@compallm/storybook/styles.css'
import {
  UrlControllerContext,
  AuthControllerContext,
  UserPreferenceControllerContext,
  AppControllerContext,
} from '@compallm/storybook/context'
import { urlController } from './url/controller'
import { authController } from './auth/controller'
import { onInvalidAuthorization } from './graphql-client'
import { userPreferenceController } from './preference/user/controller'
import { RequestProgress } from './request-progress'
import { appController } from './app/controller'
import { appearanceController } from './appearance/controller'

import { startCedarWorker } from './workers/cedar-client'
import './request-compression'

onInvalidAuthorization(authController.invalidAuthorization)
void startCedarWorker().catch(() => undefined)

function App() {
  useEffect(() => passkeyController.start(), [])
  useEffect(() => appearanceController.start(), [])
  useEffect(() => userPreferenceController.start(authController), [])
  return (
    <UrlControllerContext value={urlController}>
      <AuthControllerContext value={authController}>
        <PasskeyControllerContext value={passkeyController}>
          <UserPreferenceControllerContext value={userPreferenceController}>
            <AppControllerContext value={appController}>
              <RouterProvider router={router} />
              <PasskeyDialogs />
              <RequestProgress />
              <Toaster position="top-right" theme="dark" />
            </AppControllerContext>
          </UserPreferenceControllerContext>
        </PasskeyControllerContext>
      </AuthControllerContext>
    </UrlControllerContext>
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
