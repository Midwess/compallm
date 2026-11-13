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
import { AppLoading } from './app-loading'
import { appearanceController } from './appearance/controller'
import { ScrollMotion } from '@compallm/storybook/components/scroll'

onInvalidAuthorization(authController.invalidAuthorization)

function App() {
  useEffect(() => appearanceController.start(), [])
  useEffect(() => userPreferenceController.start(authController), [])
  return (
    <UrlControllerContext value={urlController}>
      <AuthControllerContext value={authController}>
        <UserPreferenceControllerContext value={userPreferenceController}>
          <AppControllerContext value={appController}>
            <AppLoading>
              <ScrollMotion />
              <RouterProvider router={router} />
              <RequestProgress />
              <Toaster position="top-right" theme="dark" />
            </AppLoading>
          </AppControllerContext>
        </UserPreferenceControllerContext>
      </AuthControllerContext>
    </UrlControllerContext>
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
