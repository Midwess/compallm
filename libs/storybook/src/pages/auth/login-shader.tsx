import { lazy, Suspense, useEffect, useRef, useState, type ComponentProps } from 'react'

const Warp = lazy(() =>
  import('./shader-renderer').then((module) => {
    const preset = module.warpPresets.find((entry) => entry.name === 'Passion')
    return { default: (props: ComponentProps<typeof module.Warp>) => <module.Warp {...preset?.params} {...props} /> }
  }),
)

export function LoginShader() {
  const host = useRef<HTMLDivElement>(null)
  const [colors, setColors] = useState<string[]>()
  const [playing, setPlaying] = useState(false)
  const [available, setAvailable] = useState(false)
  useEffect(() => {
    const node = host.current
    if (!node) return
    const probe = document.createElement('canvas')
    const gl = probe.getContext('webgl2', { powerPreference: 'low-power' })
    if (!gl) return
    gl.getExtension('WEBGL_lose_context')?.loseContext()
    const sample = document.createElement('canvas').getContext('2d', { willReadFrequently: true })
    const theme = () => {
      if (!sample) return
      const style = getComputedStyle(node)
      const rgb = (token: string) => {
        sample.clearRect(0, 0, 1, 1)
        sample.fillStyle = style.getPropertyValue(token).trim()
        sample.fillRect(0, 0, 1, 1)
        return Array.from(sample.getImageData(0, 0, 1, 1).data).slice(0, 3)
      }
      const background = rgb('--background')
      const foreground = rgb('--foreground')
      const base = background.reduce((a, b) => a + b, 0) < foreground.reduce((a, b) => a + b, 0) ? background : foreground
      const accent = rgb('--primary')
      const mix = (amount: number) =>
        `rgb(${base.map((channel, i) => Math.round(channel * (1 - amount) + accent[i] * amount)).join(',')})`
      setColors([mix(0.2), mix(0.06), mix(0)])
    }
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)')
    let visible = true
    const update = () => setPlaying(!document.hidden && visible && !reduced.matches)
    const observer = new MutationObserver(theme)
    const intersection = new IntersectionObserver(([entry]) => {
      visible = entry.isIntersecting
      update()
    })
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class', 'data-theme', 'data-appearance'],
    })
    intersection.observe(node)
    document.addEventListener('visibilitychange', update)
    reduced.addEventListener('change', update)
    const ready = requestAnimationFrame(() => {
      setAvailable(true)
      theme()
      update()
    })
    return () => {
      cancelAnimationFrame(ready)
      observer.disconnect()
      intersection.disconnect()
      document.removeEventListener('visibilitychange', update)
      reduced.removeEventListener('change', update)
    }
  }, [])
  return (
    <div ref={host} aria-hidden="true" className="pointer-events-none absolute inset-0 opacity-15 dark:opacity-100">
      {available && colors ? (
        <Suspense fallback={null}>
          <Warp
            width="100%"
            height="100%"
            colors={colors}
            speed={playing ? 0.8 : 0}
            frame={0}
            minPixelRatio={0.5}
            maxPixelCount={1_000_000}
            webGlContextAttributes={{ powerPreference: 'low-power', antialias: false }}
          />
        </Suspense>
      ) : null}
    </div>
  )
}
