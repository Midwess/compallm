import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from 'react-router/dom'
import { Toaster } from 'sonner'
import { router } from './App.tsx'
import '@compallm/storybook/styles.css'
import { AuthControllerContext } from '@compallm/storybook/context'
import { authController } from './auth/controller'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthControllerContext value={authController}>
      <RouterProvider router={router} />
      <Toaster position="top-right" theme="dark" />
    </AuthControllerContext>
  </StrictMode>,
)
