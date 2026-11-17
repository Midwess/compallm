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
uniform float uChanged;
uniform float uFrom;
uniform float uTarget;
uniform float uFinishing;
varying vec2 vUv;
void main() {
  float elapsed = max(0.0, uTime - uChanged);
  float duration = mix(0.5, 0.1, uFinishing);
  float t = clamp(elapsed / duration, 0.0, 1.0);
  float progress = mix(uFrom, uTarget, t * t * (3.0 - 2.0 * t));
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
  const t = Math.min(1, Math.max(0, (now - changed) / (phase === 'finishing' ? 100 : 500)))
  return from + (target - from) * (t * t * (3 - 2 * t))
}
function draw(now: number) {
  frame = 0
  if (!renderer || hidden || phase === 'idle' || renderer.gl.isContextLost()) return
  program.uniforms.uTime.value = now / 1000
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
          uChanged: { value: 0 },
          uFrom: { value: 0 },
          uTarget: { value: 0 },
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
        target = phase === 'loading' ? 0.9 : phase === 'finishing' ? 1 : 0
        changed = now
      }
      hidden = message.hidden
      reduced = message.reduced
      program.uniforms.uFrom.value = reduced ? target : from
      program.uniforms.uTarget.value = target
      program.uniforms.uChanged.value = changed / 1000
      program.uniforms.uFinishing.value = phase === 'finishing' && !reduced ? 1 : 0
    }
    resume()
  } catch {
    cancelAnimationFrame(frame)
    self.postMessage({ type: 'unavailable' })
  }
}
