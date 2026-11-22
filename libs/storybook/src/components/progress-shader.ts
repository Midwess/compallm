import { Renderer, Triangle, Program, Mesh } from 'ogl'

const vertex = `
attribute vec2 position;
attribute vec2 uv;
varying vec2 vUv;
void main() {
  vUv = uv;
  gl_Position = vec4(position, 0.0, 1.0);
}`
const fragment = `
precision mediump float;
uniform float uTime;
uniform vec3 uColor;
uniform float uElapsed;
uniform float uProgress;
uniform float uFinishing;
varying vec2 vUv;
void main() {
  float elapsed = uElapsed;
  float progress = uProgress;
  float fade = 1.0 - uFinishing * smoothstep(0.1, 0.18, elapsed);
  float mask = 1.0 - smoothstep(progress, progress + 0.001, vUv.x);
  float depth = 1.0 - vUv.y;
  float behind = max(0.0, progress - vUv.x);
  float tail = 1.0 - smoothstep(0.0, 0.32, behind);
  tail *= tail;
  float breeze = 0.5 + 0.5 * sin(behind * 24.0 + depth * 2.0 + uTime * 1.6);
  float vertical = 1.0 - depth;
  float falloff = vertical * vertical * vertical * vertical;
  float line = 1.0 - smoothstep(0.035, 0.065, depth);
  float horizontal = smoothstep(0.0, max(progress, 0.001), vUv.x);
  float curtain = vertical * (0.08 + horizontal * horizontal * 0.6);
  float edgeGlow = falloff * tail * (0.35 + breeze * 0.06);
  float alpha = max(line, curtain + edgeGlow);
  vec3 color = mix(uColor, vec3(1.0), line * tail * 0.18);
  gl_FragColor = vec4(color, alpha * mask * fade);
}`

export type ProgressPhase = 'idle' | 'loading' | 'finishing'
export type ProgressMessage =
  | { type: 'init'; canvas: OffscreenCanvas }
  | { type: 'resize'; width: number }
  | { type: 'theme'; color: number[] }
  | { type: 'state'; phase: ProgressPhase; hidden: boolean; reduced: boolean }

let renderer: Renderer
let program: Program
let mesh: Mesh
let phase: ProgressPhase = 'idle'
let hidden = false
let reduced = false
let frame = 0
let changed = 0
let from = 0
let target = 0

function progress(now: number) {
  const elapsed = Math.max(0, now - changed)
  if (phase === 'idle') return 0
  if (phase === 'finishing') {
    const t = Math.min(1, elapsed / 100)
    return from + (1 - from) * (t * t * (3 - 2 * t))
  }
  const knee = Math.max(from, 0.7)
  if (from < 0.7 && elapsed < 700) {
    const t = elapsed / 700
    return from + (knee - from) * (t * t * (3 - 2 * t))
  }
  const tail = Math.max(0, elapsed - (from < 0.7 ? 700 : 0))
  return knee + ((Math.max(0.98, knee) - knee) * tail) / (tail + 30000)
}
function draw(now: number) {
  frame = 0
  if (!renderer || hidden || phase === 'idle' || renderer.gl.isContextLost()) return
  program.uniforms.uTime.value = (now - changed) / 1000
  program.uniforms.uElapsed.value = (now - changed) / 1000
  program.uniforms.uProgress.value = reduced ? target : progress(now)
  renderer.render({ scene: mesh })
  if (!reduced && (phase === 'loading' || now - changed < 180)) frame = requestAnimationFrame(draw)
}
function resume() {
  cancelAnimationFrame(frame)
  draw(performance.now())
}
self.onmessage = (event: MessageEvent<ProgressMessage>) => {
  const message = event.data
  try {
    if (message.type === 'init') {
      renderer = new Renderer({
        canvas: message.canvas as unknown as HTMLCanvasElement,
        alpha: true,
        premultipliedAlpha: false,
        antialias: false,
        dpr: 1,
        powerPreference: 'low-power',
      })
      const gl = renderer.gl
      if (!gl) throw new Error('WebGL unavailable')
      program = new Program(gl, {
        vertex,
        fragment,
        transparent: true,
        depthTest: false,
        depthWrite: false,
        uniforms: {
          uTime: { value: 0 },
          uColor: { value: [0.5, 0.4, 1] },
          uElapsed: { value: 0 },
          uProgress: { value: 0 },
          uFinishing: { value: 0 },
        },
      })
      mesh = new Mesh(gl, { geometry: new Triangle(gl), program })
      message.canvas.addEventListener('webglcontextlost', () => {
        cancelAnimationFrame(frame)
        self.postMessage({ type: 'unavailable' })
      })
      self.postMessage({ type: 'ready' })
    } else if (message.type === 'resize') {
      renderer.setSize(message.width, 40)
    } else if (message.type === 'theme') {
      program.uniforms.uColor.value = message.color
    } else {
      const now = performance.now()
      if (phase !== message.phase) {
        from = phase === 'idle' ? 0.08 : progress(now)
        phase = message.phase
        target = phase === 'loading' ? 0.7 : phase === 'finishing' ? 1 : 0
        changed = now
      }
      hidden = message.hidden
      reduced = message.reduced
      program.uniforms.uFinishing.value = phase === 'finishing' && !reduced ? 1 : 0
    }
    resume()
  } catch {
    cancelAnimationFrame(frame)
    self.postMessage({ type: 'unavailable' })
  }
}
