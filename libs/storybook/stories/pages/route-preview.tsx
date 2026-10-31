import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { AuthDemo } from './auth-demo'
import '../../.storybook/tailwind.css'

const pathname = window.location.pathname.replace(/\/$/, '')
const mode = pathname === '/signin' ? 'signin' : 'signup'
document.title = mode === 'signup' ? 'Create your account · Compallm' : 'Sign in · Compallm'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthDemo mode={mode} />
  </StrictMode>,
)
