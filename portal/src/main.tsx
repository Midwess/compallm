import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router/dom'
import { Toaster } from 'sonner'
import { router } from './App.tsx'
import '@compallm/storybook/styles.css'
import { UrlControllerContext, AuthControllerContext } from '@compallm/storybook/context'
import { urlController } from './url/controller'
import { authController } from './auth/controller'
import { onInvalidAuthorization } from './graphql-client'
import { RequestProgress } from './request-progress'

onInvalidAuthorization(authController.invalidAuthorization)

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <UrlControllerContext value={urlController}>
      <AuthControllerContext value={authController}>
        <RouterProvider router={router} />
        <RequestProgress />
        <Toaster position="top-right" theme="dark" />
      </AuthControllerContext>
    </UrlControllerContext>
  </StrictMode>,
)
