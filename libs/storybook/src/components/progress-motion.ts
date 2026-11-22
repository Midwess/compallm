export const progressTiming = {
  minimumVisible: 500,
  grace: 0,
  approach: 700,
  knee: 0.7,
  ceiling: 0.99,
  tail: 30000,
  finish: 640,
  settle: 80,
  fade: 240,
} as const

export type ProgressPhase = 'idle' | 'loading' | 'finishing'
export interface ProgressSnapshot {
  phase: ProgressPhase
  changed: number
  from: number
  tangent: number
}

export class ProgressMotion {
  private phase: ProgressPhase = 'idle'
  private changed = 0
  private from = 0
  private tangent = 0

  setPhase(phase: ProgressPhase, now: number) {
    if (phase === this.phase) return
    const value = this.value(now)
    const velocity = value - this.value(now - 1)
    this.from = this.phase === 'idle' || (phase === 'loading' && value >= 0.999) ? 0.08 : value
    this.tangent = Math.min(3, Math.max(0, (velocity * progressTiming.finish) / Math.max(0.001, 1 - this.from)))
    this.phase = phase
    this.changed = now
  }

  snapshot(offset = 0): ProgressSnapshot {
    return { phase: this.phase, changed: this.changed + offset, from: this.from, tangent: this.tangent }
  }

  restore(value: ProgressSnapshot, offset = 0) {
    this.phase = value.phase
    this.changed = value.changed - offset
    this.from = value.from
    this.tangent = value.tangent
  }

  value(now: number): number {
    const elapsed = Math.max(0, now - this.changed)
    if (this.phase === 'idle') return 0
    if (this.phase === 'finishing') {
      const t = Math.min(1, elapsed / progressTiming.finish)
      const eased = t * t * (3 - 2 * t) + this.tangent * t * (1 - t) * (1 - t)
      return this.from + (1 - this.from) * eased
    }
    const knee = Math.max(this.from, progressTiming.knee)
    if (this.from < knee && elapsed < progressTiming.approach) {
      const t = elapsed / progressTiming.approach
      return this.from + (knee - this.from) * t * t * (3 - 2 * t)
    }
    const tail = Math.max(0, elapsed - (this.from < knee ? progressTiming.approach : 0))
    return knee + ((Math.max(progressTiming.ceiling, (1 + knee) / 2) - knee) * tail) / (tail + progressTiming.tail)
  }

  opacity(now: number): number {
    if (this.phase === 'idle') return 0
    if (this.phase !== 'finishing') return 1
    const t = Math.min(
      1,
      Math.max(0, (now - this.changed - progressTiming.finish - progressTiming.settle) / progressTiming.fade),
    )
    return 1 - t * t * (3 - 2 * t)
  }

  get duration() {
    return progressTiming.finish + progressTiming.settle + progressTiming.fade
  }
}
