import { type FormEvent, useState } from 'react'
import { confirmSignupOtp, signupEmail } from './contract/api'
import './App.css'

function App() {
  const [email, setEmail] = useState('')
  const [awaitingCode, setAwaitingCode] = useState(false)
  const [code, setCode] = useState('')
  const [isPending, setIsPending] = useState(false)
  const [isComplete, setIsComplete] = useState(false)
  const [error, setError] = useState<string | null>(null)

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (isPending) return
    setIsPending(true)
    setError(null)
    try {
      if (awaitingCode) {
        await confirmSignupOtp({ code })
        setAwaitingCode(false)
        setCode('')
        setIsComplete(true)
      } else {
        await signupEmail({ email })
        setAwaitingCode(true)
      }
    } catch (caught) {
      const failure = caught as { info?: { message?: string } }
      setError(failure.info?.message ?? 'The request could not be completed. Please try again.')
    } finally {
      setIsPending(false)
    }
  }

  return (
    <main className="shell">
      <header className="masthead">
        <a className="wordmark" href="/">Compallm</a>
      </header>
      <section className="hero-grid">
        <div className="intro">
          <p className="eyebrow">Your organization workspace</p>
          <h1>Work with LLMs, together.</h1>
          <p className="lede">Create your account with your email. No password required.</p>
        </div>
        <section className="console" aria-labelledby="signup-title">
          <div className="console-heading">
            <h2 id="signup-title">{isComplete ? 'Account created' : awaitingCode ? 'Check your email' : 'Create an account'}</h2>
          </div>
          {isComplete ? (
            <p>Your email is verified and your account is ready.</p>
          ) : (
            <form onSubmit={handleSubmit}>
              {awaitingCode ? (
                <>
                  <p>Enter the six-digit code sent to {email}. It expires in 10 minutes.</p>
                  <label htmlFor="code">Email code</label>
                  <div className="input-row">
                    <input id="code" name="code" value={code}
                      onChange={(event) => setCode(event.target.value)}
                      autoComplete="one-time-code" inputMode="numeric"
                      pattern="[0-9]{6}" maxLength={6} required disabled={isPending} />
                  </div>
                </>
              ) : (
                <>
                  <label htmlFor="email">Email address</label>
                  <div className="input-row">
                    <input id="email" name="email" type="email" value={email}
                      onChange={(event) => setEmail(event.target.value)}
                      autoComplete="email" required disabled={isPending} />
                  </div>
                </>
              )}
              <div className="input-row">
                <button disabled={isPending} type="submit">
                  {isPending ? 'Please wait…' : awaitingCode ? 'Confirm email' : 'Send email code'}
                </button>
              </div>
              {awaitingCode ? (
                <button className="secondary-action" disabled={isPending} type="button"
                  onClick={() => { setAwaitingCode(false); setCode(''); setError(null) }}>
                  Change email or request another code
                </button>
              ) : null}
            </form>
          )}
          {error ? <p className="result error" role="alert">{error}</p> : null}
        </section>
      </section>
    </main>
  )
}

export default App
