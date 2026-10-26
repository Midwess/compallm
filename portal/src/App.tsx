import { type FormEvent, useState } from 'react'
import { confirmSignupOtp, signupEmail } from './contract/api'
import './App.css'

function App() {
  const [email, setEmail] = useState('')
  const [token, setToken] = useState<string | null>(null)
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
      if (token) {
        await confirmSignupOtp({ code }, { headers: { Authorization: `Bearer ${token}` } })
        setToken(null)
        setCode('')
        setIsComplete(true)
      } else {
        const challenge = await signupEmail({ email })
        setToken(challenge.token)
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
            <h2 id="signup-title">{isComplete ? 'Account created' : token ? 'Check your email' : 'Create an account'}</h2>
          </div>
          {isComplete ? (
            <p>Your email is verified and your account is ready.</p>
          ) : (
            <form onSubmit={handleSubmit}>
              {token ? (
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
                  {isPending ? 'Please wait…' : token ? 'Confirm email' : 'Send email code'}
                </button>
              </div>
              {token ? (
                <button className="secondary-action" disabled={isPending} type="button"
                  onClick={() => { setToken(null); setCode(''); setError(null) }}>
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
