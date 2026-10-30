import type { ReactNode } from 'react'
import './preview.css'

export function Preview({ children }: { children: ReactNode }) { return <main className="component-preview">{children}</main> }
export function Sample({ label, children, wide = false }: { label: string; children: ReactNode; wide?: boolean }) {
  return <section className={`component-sample${wide ? ' component-sample--wide' : ''}`}><h3>{label}</h3><div className="component-sample-content">{children}</div></section>
}
export function Row({ children }: { children: ReactNode }) { return <div className="component-row">{children}</div> }
