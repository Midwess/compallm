const dimension = new RegExp(
  '(?<![\\w-])-?(?:min-w|min-h|max-w|max-h|size|w|h|p[xytrblse]?|m[xytrblse]?|' +
    'gap(?:-[xy])?|top|bottom|left|right|inset(?:-[xy])?|space-[xy])-' +
    '(?:\\[(\\d+(?:\\.\\d+)?)px\\]|(\\d+(?:\\.\\d+)?)|(px))(?![\\w./%-])',
  'g',
)

export default {
  meta: {
    type: 'suggestion',
    messages: {
      spacing: 'Use Tailwind spacing in half steps; explain design or functional exceptions with a local suppression.',
    },
    schema: [],
  },
  create(context) {
    function check(node, value) {
      dimension.lastIndex = 0
      for (const match of value.matchAll(dimension)) {
        if (match[1] || match[3] || (Number(match[2]) * 2) % 1 !== 0) {
          context.report({ node, messageId: 'spacing' })
          break
        }
      }
    }
    return {
      Literal(node) {
        if (typeof node.value === 'string') check(node, node.value)
      },
      TemplateElement(node) {
        check(node, node.value.raw)
      },
    }
  },
}
