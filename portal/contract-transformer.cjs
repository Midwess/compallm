// Zod counts UTF-16 code units; Validify/OpenAPI count Unicode scalar values.
// The generated wrapper applies string length constraints after normalization.
/* global module */
module.exports = function transform(document) {
  function visit(value) {
    if (!value || typeof value !== 'object') return
    if (value.type === 'string') {
      delete value.minLength
      delete value.maxLength
    }
    Object.values(value).forEach(visit)
  }
  visit(document)
  return document
}
