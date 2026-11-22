import { ProgressMotion, progressTiming, type ProgressPhase, type ProgressSnapshot } from './progress-motion'
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
precision highp float;
uniform float uTime;
uniform vec3 uColor;
uniform float uProgress;
uniform float uOpacity;
varying vec2 vUv;
void main() {
  float progress = uProgress;
  float fade = uOpacity;
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

export type ProgressMessage =
  | { type: 'init'; canvas: OffscreenCanvas }
  | { type: 'resize'; width: number }
  | { type: 'theme'; color: number[] }
  | { type: 'state'; phase: ProgressPhase; hidden: boolean; reduced: boolean; motion: ProgressSnapshot }

let renderer: Renderer
let program: Program
let mesh: Mesh
let phase: ProgressPhase = 'idle'
let hidden = false
let reduced = false
let frame = 0
const timeline = new ProgressMotion()
function draw(now: number) {
  frame = 0
  if (!renderer || hidden || phase === 'idle' || renderer.gl.isContextLost()) return
  program.uniforms.uTime.value = (now / 1000) % (Math.PI * 10)
  program.uniforms.uProgress.value = reduced ? (phase === 'loading' ? progressTiming.knee : 1) : timeline.value(now)
  program.uniforms.uOpacity.value = reduced ? 1 : timeline.opacity(now)
  renderer.render({ scene: mesh })
  if (!reduced && (phase === 'loading' || timeline.opacity(now) > 0)) frame = requestAnimationFrame(draw)
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
          uProgress: { value: 0 },
          uOpacity: { value: 1 },
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
      timeline.restore(message.motion, performance.timeOrigin)
      phase = message.phase
      hidden = message.hidden
      reduced = message.reduced
    }
    resume()
  } catch {
    cancelAnimationFrame(frame)
    self.postMessage({ type: 'unavailable' })
  }
}
