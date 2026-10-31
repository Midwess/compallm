import { Profiler, useRef, useState } from 'react'
import { AuthControllerProvider } from '../../src/pages/auth/context'
import { EmailInput, ContinueButton, CodeInput, ResendControl } from '../../src/pages/auth/auth-page'
import { DemoAuthController } from './demo-auth-controller'

class SubscriptionDemoController extends DemoAuthController {
  advanceCountdown = () => this.store.setState({ remaining: this.store.getState().remaining + 1 })
}

export function ControllerSubscriptions() {
  const [controller] = useState(() => new SubscriptionDemoController('signup'))
  const emailRef = useRef<HTMLInputElement>(null)
  const counters = useRef<Record<string, HTMLOutputElement | null>>({})
  function committed(id: string) {
    const counter = counters.current[id]
    if (counter) counter.textContent = String(Number(counter.textContent) + 1)
  }
  return (
    <AuthControllerProvider controller={controller}>
      <main className="grid max-w-[720px] gap-4 p-4 font-sans text-text">
        <h1>Controller subscriptions</h1>
        <p>One controller, four independent UI subscriptions. Counts record React commits.</p>
        {['email', 'continue', 'code', 'resend'].map((id) => (
          <p key={id}>
            {id}:{' '}
            <output
              aria-label={`${id} renders`}
              ref={(element) => {
                counters.current[id] = element
              }}
            >
              0
            </output>
          </p>
        ))}
        <label htmlFor="subscription-email">Email address</label>
        <Profiler id="email" onRender={committed}>
          <EmailInput id="subscription-email" errorId="subscription-error" inputRef={emailRef} />
        </Profiler>
        <Profiler id="continue" onRender={committed}>
          <ContinueButton />
        </Profiler>
        <Profiler id="code" onRender={committed}>
          <CodeInput errorId="subscription-error" />
        </Profiler>
        <Profiler id="resend" onRender={committed}>
          <ResendControl />
        </Profiler>
        <button type="button" onClick={controller.advanceCountdown}>
          Advance countdown
        </button>
      </main>
    </AuthControllerProvider>
  )
}
