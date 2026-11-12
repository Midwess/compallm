const token =
  /\?(?:principal|resource)|[_a-zA-Z][_a-zA-Z0-9]*|[0-9]+|"(?:\\[^\n]|[^"\\])*"|::|==|!=|<=|>=|\|\||&&|[@.,;:(){}\[\]<>+*/%!\-]/uy
const whitespace = /\p{White_Space}+/uy

function tokens(source: string): string[] | undefined {
  const result: string[] = []
  let index = 0
  while (index < source.length) {
    whitespace.lastIndex = index
    const space = whitespace.exec(source)
    if (space) {
      index = whitespace.lastIndex
      continue
    }
    if (source.startsWith('//', index)) {
      while (index < source.length && source[index] !== '\n' && source[index] !== '\r') index++
      continue
    }
    token.lastIndex = index
    const match = token.exec(source)
    if (!match) return undefined
    result.push(match[0])
    index = token.lastIndex
  }
  return result
}

export function compactCedarSource(source: string): string {
  const values = tokens(source)
  if (!values) return source
  let result = ''
  for (let index = 0; index < values.length; index++) {
    if (index > 0) {
      const pair = tokens(values[index - 1] + values[index])
      if (!pair || pair.length !== 2 || pair[0] !== values[index - 1] || pair[1] !== values[index]) result += ' '
    }
    result += values[index]
  }
  return result
}
