import type { ReactNode } from 'react'
import { BookOpenIcon } from '@phosphor-icons/react/dist/csr/BookOpen'
import { SparkleIcon } from '@phosphor-icons/react/dist/csr/Sparkle'

export function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <main
      className={`
  grid min-h-svh grid-cols-[61.8%_38.2%] bg-background font-sans text-text max-[800px]:block
`}
    >
      <section
        className={`
  relative flex items-center justify-center bg-surface px-space-64 py-space-128 before:pointer-events-none
  before:absolute before:inset-0 before:bg-[radial-gradient(var(--color-disabled)_0.5px,transparent_0.5px)]
  before:bg-size-[80px_80px] before:opacity-30 before:content-[''] max-[800px]:hidden
`}
        data-ui="auth-introduction"
        aria-label="Compallm"
      >
        <div
          className={`
  absolute top-space-64 left-space-64 flex items-center gap-space-32 text-[22px] leading-[32px] font-medium
  [&>svg]:text-focus
`}
        >
          <BookOpenIcon size={24} aria-hidden="true" />
          <span>Compallm</span>
        </div>
        <div
          className={`
  z-1 w-full max-w-[740px] text-center [&>h2]:m-0 [&>h2]:text-[clamp(32px,3.34vw,48px)]
  [&>h2]:leading-[1.25] [&>h2]:font-medium [&>h2]:tracking-[-1.2px] [&>p]:mt-space-32
  [&>p]:text-[clamp(16px,1.4vw,20px)] [&>p]:leading-[32px] [&>p]:text-muted
`}
        >
          <SparkleIcon className="mx-auto mb-space-64 block text-focus" size={49} aria-hidden="true" />
          <h2>
            More room to think.
            <br />
            One place to begin.
          </h2>
          <p>
            Bring your models, conversations, and knowledge
            <br />
            together in one workspace.
          </p>
        </div>
      </section>
      <section
        className={`
  flex min-w-0 flex-col items-center justify-center p-space-64 max-[800px]:relative max-[800px]:min-h-svh
  max-[800px]:px-space-32 max-[800px]:py-space-128
`}
        aria-label="Account access"
      >
        <div
          className={`
  hidden items-center gap-space-32 text-[22px] leading-[32px] font-medium max-[800px]:absolute
  max-[800px]:top-space-32 max-[800px]:left-space-32 max-[800px]:flex [&>svg]:text-focus
`}
        >
          <BookOpenIcon size={24} aria-hidden="true" />
          <span>Compallm</span>
        </div>
        <div className="-mt-space-64 w-[352px] max-w-full max-[800px]:mt-0">{children}</div>
      </section>
    </main>
  )
}
