export function cx(...values: Array<string | undefined | false>) {
  return values.filter(Boolean).join(' ')
}
