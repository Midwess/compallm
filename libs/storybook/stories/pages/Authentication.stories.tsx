import { useEffect, useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { BookOpenIcon } from '@phosphor-icons/react/dist/csr/BookOpen'
import { FingerprintIcon } from '@phosphor-icons/react/dist/csr/Fingerprint'
import { Button } from '../../src/components/buttons'
import { AuthControllerContext } from '../../src/context'
import { AccessLoginPage, AccessVerificationDialog } from '../../src/pages/auth/access-panel'
import type { AuthState } from '../../src/pages/auth/types'
import { DemoAuthController } from './demo-auth-controller'

class AccessPreviewController extends DemoAuthController {
  constructor(
    private readonly waiting: boolean,
    initial: Partial<AuthState>,
    mode: 'signin' | 'signup' = 'signin',
  ) {
    super(mode)
    this.store.setState({ ...(mode === 'signup' ? { step: 'email' as const } : {}), ...initial })
    const verifyEmail = this.verifyCode
    this.verifyCode = async (code: string) => {
      await verifyEmail(code)
      if (this.store.getState().step === 'complete') this.store.setState({ lastLoginMethod: 'email' })
    }
  }
  verifyPasskey = async () => {
    this.store.setState({ step: 'passkey', pending: 'passkey', error: null, serverError: null })
    if (this.waiting) return
    this.store.setState({ step: 'complete', pending: null, lastLoginMethod: 'passkey' })
  }
}

function Preview({
  signup = false,
  popup = false,
  initial = {},
  waiting = false,
}: {
  signup?: boolean
  popup?: boolean
  initial?: Partial<AuthState>
  waiting?: boolean
}) {
  const [controller] = useState(() => new AccessPreviewController(waiting, initial, signup ? 'signup' : 'signin'))
  const [open, setOpen] = useState(true)
  useEffect(() => controller.start(), [controller])
  return (
    <AuthControllerContext value={controller}>
      {popup ? (
        <main className="min-h-svh bg-background p-6 text-foreground sm:p-12">
          <div className="mx-auto max-w-200">
            <div className="mb-14 flex items-center gap-2 text-sm font-medium">
              <BookOpenIcon size={20} />
              Compallm
            </div>
            <div className="mb-8 text-xs text-muted-foreground">
              Account settings <span className="mx-2">/</span> Security & access
            </div>
            <h1 className="text-2xl font-medium tracking-tight">Security & access</h1>
            <p className="mt-2 text-sm text-muted-foreground">Choose how you access your account.</p>
            <section className="mt-8 rounded-xl border border-border bg-inset p-5">
              <div className="flex flex-wrap items-center justify-between gap-4">
                <div className="flex items-center gap-3">
                  <FingerprintIcon size={22} />
                  <div>
                    <h2 className="text-sm font-medium">Passkeys</h2>
                    <p className="mt-1 text-xs text-muted-foreground">Sign in with your fingerprint, face, or device.</p>
                  </div>
                </div>
                <Button
                  variant="neutral"
                  onClick={() => {
                    controller.backToMethods()
                    setOpen(true)
                  }}
                >
                  New passkey
                </Button>
              </div>
            </section>
          </div>
          <AccessVerificationDialog open={open} onOpenChange={setOpen} accountEmail="owner@compallm.com" />
        </main>
      ) : (
        <AccessLoginPage mode={signup ? 'signup' : 'login'} />
      )}
    </AuthControllerContext>
  )
}

const meta = {
  title: 'Pages/Authentication',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        component:
          'Shared login and identity-verification design. Both use AuthControllerContext. ' +
          'Offline preview: use 123456 for email verification. ' +
          'Last-used method is retained by the preview controller; close and reopen verification to see it. ' +
          'No credentials or emails are sent.',
      },
    },
  },
} satisfies Meta
export default meta
type Story = StoryObj<typeof meta>

export const Login: Story = { render: () => <Preview /> }
export const LoginLight: Story = { globals: { theme: 'BASIC_WHITE' }, render: () => <Preview /> }
export const LoginMobile: Story = {
  globals: { viewport: { value: 'mobile2', isRotated: false } },
  render: () => <Preview />,
}
export const ReturningWithEmail: Story = { render: () => <Preview initial={{ lastLoginMethod: 'email' }} /> }
export const ReturningWithPasskey: Story = { render: () => <Preview initial={{ lastLoginMethod: 'passkey' }} /> }
export const EmailEntry: Story = { render: () => <Preview initial={{ step: 'email' }} /> }
export const EmailCode: Story = {
  render: () => <Preview initial={{ step: 'code', email: 'alex@example.com', remaining: 24 }} />,
}
export const EmailCodeError: Story = {
  render: () => <Preview initial={{ step: 'code', email: 'alex@example.com', error: 'incorrect-code' }} />,
}
export const WaitingForPasskey: Story = {
  render: () => <Preview waiting initial={{ step: 'passkey', pending: 'passkey', lastLoginMethod: 'passkey' }} />,
}
export const PasskeyCancelled: Story = {
  render: () => (
    <Preview
      initial={{
        lastLoginMethod: 'passkey',
        serverError: 'Passkey verification was cancelled. Try again or continue with email.',
      }}
    />
  ),
}
export const VerifyIdentity: Story = {
  render: () => <Preview popup initial={{ lastLoginMethod: 'passkey', email: 'owner@compallm.com' }} />,
}
export const VerifyWithEmail: Story = {
  render: () => <Preview popup initial={{ step: 'code', email: 'owner@compallm.com', remaining: 24 }} />,
}
export const Verified: Story = { render: () => <Preview popup initial={{ step: 'complete', lastLoginMethod: 'email' }} /> }

export const Signup: Story = { render: () => <Preview signup /> }
export const SignupEmailCode: Story = {
  render: () => <Preview signup initial={{ step: 'code', email: 'alex@example.com', remaining: 24 }} />,
}
