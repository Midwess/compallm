export type TokenSelection = { start: number; end: number }
export type TokenEntryState =
  | { kind: 'preview'; selection?: TokenSelection }
  | { kind: 'editing'; groupedUntil: number; compositionCommit?: string }
  | { kind: 'composing'; groupedUntil: number; draft: string }
export type TokenEntryEvent =
  | { type: 'focus'; selection?: TokenSelection }
  | { type: 'input'; text: string; selection: TokenSelection }
  | { type: 'composition-start'; text: string }
  | { type: 'composition-end'; text: string; selection: TokenSelection }
  | { type: 'blur'; selection: TokenSelection }
  | { type: 'backspace'; selection: TokenSelection }
export type TokenEntryTransition = {
  state: TokenEntryState
  value?: string
  selection?: TokenSelection
  validate?: boolean
}
export function tokenParts(value: string, parse: (text: string) => string | null) {
  return Array.from(value.matchAll(/[^\s;,|]+/gu), (match) => ({
    text: match[0],
    start: match.index,
    end: match.index + match[0].length,
    token: parse(match[0]),
  }))
}
function normalized(value: string) {
  return value.trim()
}
function offset(value: string, position: number) {
  return Math.max(0, position - (value.length - value.trimStart().length))
}
export function transitionTokenEntry(
  state: TokenEntryState,
  event: TokenEntryEvent,
  value: string,
  parse: (text: string) => string | null,
): TokenEntryTransition {
  const groupedUntil = state.kind === 'preview' ? 0 : Math.min(state.groupedUntil, value.length)
  switch (event.type) {
    case 'focus':
      return {
        state: { kind: 'editing', groupedUntil: 0 },
        selection: event.selection ?? (state.kind === 'preview' ? state.selection : undefined),
      }
    case 'composition-start':
      return { state: { kind: 'composing', groupedUntil, draft: event.text } }
    case 'composition-end':
    case 'input': {
      if (event.type === 'input' && state.kind === 'composing') return { state: { ...state, draft: event.text } }
      if (event.type === 'input' && state.kind === 'editing' && state.compositionCommit === event.text)
        return { state: { kind: 'editing', groupedUntil } }
      const composition = event.type === 'composition-end' ? { compositionCommit: event.text } : {}
      const next = value.slice(0, groupedUntil) + event.text
      const position = groupedUntil + event.selection.start
      const completed =
        event.selection.start === event.selection.end && position === next.length ? next.match(/([^\s;,|]+)[\s;,|]+$/u) : null
      if (completed && parse(completed[1])) {
        const text = next.trimStart()
        return {
          state: { kind: 'editing', groupedUntil: text.length, ...composition },
          value: text,
          selection: { start: 0, end: 0 },
          validate: true,
        }
      }
      return { state: { kind: 'editing', groupedUntil, ...composition }, value: next }
    }
    case 'blur': {
      const next = normalized(value)
      return {
        state: {
          kind: 'preview',
          selection: {
            start: Math.min(next.length, offset(value, groupedUntil + event.selection.start)),
            end: Math.min(next.length, offset(value, groupedUntil + event.selection.end)),
          },
        },
        value: next,
        validate: true,
      }
    }
    case 'backspace': {
      if (state.kind !== 'editing' || !groupedUntil || event.selection.start || event.selection.end) return { state }
      const previous = value.slice(0, groupedUntil).match(/([^\s;,|]+)[\s;,|]*$/u)
      if (!previous) return { state }
      return {
        state: { kind: 'editing', groupedUntil: previous.index! },
        selection: { start: previous[1].length, end: previous[1].length },
      }
    }
  }
}
