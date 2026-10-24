import { type FormEvent, useState } from 'react'
import {
  createGreeting,
  getGreeting,
  type Greeting,
} from './contract/api'
import './App.css'

function App() {
  const [name, setName] = useState('Ada')
  const [greeting, setGreeting] = useState<Greeting | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [isPending, setIsPending] = useState(false)

  async function runServerFunction(action: () => Promise<Greeting>) {
    setIsPending(true)
    setError(null)

    try {
      setGreeting(await action())
    } catch (caught) {
      const requestError = caught as Error & { status?: number }
      setError(
        caught instanceof Error
          ? requestError.status
            ? `Loco returned HTTP ${requestError.status}.`
            : `Loco request failed: ${caught.message}`
          : 'Could not reach the Loco server.',
      )
    } finally {
      setIsPending(false)
    }
  }

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    void runServerFunction(() => createGreeting({ name }))
  }

  return (
    <main className="shell">
      <header className="masthead">
        <a className="wordmark" href="/" aria-label="Loco and Vite home">
          <span>A</span>
          <span>+</span>
          <span>V</span>
        </a>
        <div className="runtime-badges" aria-label="Runtime architecture">
          <span>Loco owns port 3000</span>
          <span>Vite HMR via proxy</span>
        </div>
      </header>

      <section className="hero-grid">
        <div className="intro">
          <p className="eyebrow">One origin. Two toolchains.</p>
          <h1>Rust functions, called like local TypeScript.</h1>
          <p className="lede">
            Loco and Axum own the API and public port. During development they
            forward the UI to Vite, including the Fast Refresh WebSocket. In
            production, they serve Vite's compiled files directly.
          </p>

          <div className="flow" aria-label="Request flow">
            <span>React</span>
            <b aria-hidden="true">-&gt;</b>
            <span>generated function</span>
            <b aria-hidden="true">-&gt;</b>
            <span>Loco</span>
          </div>
        </div>

        <section className="console" aria-labelledby="console-title">
          <div className="console-heading">
            <div>
              <p className="kicker">Live contract</p>
              <h2 id="console-title">Call Loco</h2>
            </div>
            <span className="status-dot">connected through /api</span>
          </div>

          <form onSubmit={handleSubmit}>
            <label htmlFor="name">Name sent to Rust</label>
            <div className="input-row">
              <input
                id="name"
                name="name"
                value={name}
                onChange={(event) => setName(event.target.value)}
                autoComplete="name"
              />
              <button disabled={isPending} type="submit">
                {isPending ? 'Calling...' : 'createGreeting()'}
              </button>
            </div>
          </form>

          <button
            className="secondary-action"
            disabled={isPending}
            type="button"
            onClick={() => void runServerFunction(getGreeting)}
          >
            Or call getGreeting()
          </button>

          <output className={error ? 'result error' : 'result'} aria-live="polite">
            {error ? (
              <span>{error}</span>
            ) : greeting ? (
              <>
                <strong>{greeting.message}</strong>
                <span>
                  Served by {greeting.servedBy}; request #{greeting.requestCount}
                </span>
              </>
            ) : (
              <span>The generated response will appear here.</span>
            )}
          </output>
        </section>
      </section>

      <section className="notes" aria-label="Project conventions">
        <article>
          <span>01</span>
          <h2>Edit the UI</h2>
          <p>
            Change <code>portal/src/App.tsx</code>. Vite updates the browser without
            restarting Loco.
          </p>
        </article>
        <article>
          <span>02</span>
          <h2>Edit the contract</h2>
          <p>
            Annotate handlers in <code>api/src/controllers/</code>. Utoipa emits
            OpenAPI, then Orval regenerates <code>portal/src/contract/api.ts</code>.
          </p>
        </article>
        <article>
          <span>03</span>
          <h2>Ship one server</h2>
          <p>
            Build Vite, then run the Rust release binary. Loco serves the API and
            the static application.
          </p>
        </article>
      </section>
    </main>
  )
}

export default App
