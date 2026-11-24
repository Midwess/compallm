<script lang="ts">
  import { onMount } from 'svelte'
  let host: HTMLDivElement
  onMount(() => {
    let disposed = false
    let shader: import('@paper-design/shaders').ShaderMount | undefined
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)')
    let visible = true
    const update = () => shader?.setSpeed(!document.hidden && visible && !reduced.matches ? 0.8 : 0)
    const sample = document.createElement('canvas').getContext('2d', { willReadFrequently: true })
    const colors = () => {
      if (!sample) return []
      const style = getComputedStyle(host)
      const rgb = (token: string) => {
        sample.clearRect(0, 0, 1, 1)
        sample.fillStyle = style.getPropertyValue(token).trim()
        sample.fillRect(0, 0, 1, 1)
        return Array.from(sample.getImageData(0, 0, 1, 1).data).slice(0, 3)
      }
      const background = rgb('--background'),
        foreground = rgb('--foreground')
      const base = background.reduce((a, b) => a + b, 0) < foreground.reduce((a, b) => a + b, 0) ? background : foreground
      const accent = rgb('--primary')
      return [0.2, 0.06, 0].map(
        (amount) =>
          `rgb(${base.map((channel, index) => Math.round(channel * (1 - amount) + accent[index] * amount)).join(',')})`,
      )
    }
    let parseColor: typeof import('@paper-design/shaders').getShaderColorFromString | undefined
    const theme = () => {
      if (shader && parseColor) shader.setUniforms({ u_colors: colors().map(parseColor) })
    }
    const observer = new MutationObserver(theme)
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class', 'data-theme', 'data-appearance'],
    })
    const intersection = new IntersectionObserver(([entry]) => {
      visible = entry.isIntersecting
      update()
    })
    intersection.observe(host)
    document.addEventListener('visibilitychange', update)
    reduced.addEventListener('change', update)
    const ready = requestAnimationFrame(() => {
      void (async () => {
        const probe = document.createElement('canvas')
        const gl = probe.getContext('webgl2', { powerPreference: 'low-power' })
        if (!gl) return
        gl.getExtension('WEBGL_lose_context')?.loseContext()
        const module = await import('@paper-design/shaders')
        const noise = module.getShaderNoiseTexture()
        await noise?.decode()
        if (disposed || !sample) return
        parseColor = module.getShaderColorFromString
        shader = new module.ShaderMount(
          host,
          module.warpFragmentShader,
          {
            u_colors: colors().map(parseColor),
            u_colorsCount: 3,
            u_proportion: 0.5,
            u_softness: 1,
            u_distortion: 0.09,
            u_swirl: 0.9,
            u_swirlIterations: 6,
            u_shapeScale: 0.25,
            u_shape: module.WarpPatterns.checks,
            u_noiseTexture: noise,
            u_scale: 2.5,
            u_rotation: 1.35,
            u_fit: module.ShaderFitOptions.none,
            u_offsetX: 0,
            u_offsetY: 0,
            u_originX: 0.5,
            u_originY: 0.5,
            u_worldWidth: 0,
            u_worldHeight: 0,
          },
          { powerPreference: 'low-power', antialias: false },
          0,
          0,
          0.5,
          1_000_000,
        )
        update()
      })().catch(() => {
        shader?.dispose()
        shader = undefined
      })
    })
    return () => {
      disposed = true
      cancelAnimationFrame(ready)
      shader?.dispose()
      observer.disconnect()
      intersection.disconnect()
      document.removeEventListener('visibilitychange', update)
      reduced.removeEventListener('change', update)
    }
  })
</script>

<div bind:this={host} aria-hidden="true" class="pointer-events-none absolute inset-0 opacity-15 dark:opacity-100"></div>
