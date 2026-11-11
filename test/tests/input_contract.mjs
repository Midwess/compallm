import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { test } from 'node:test'
import {
  ProviderFields,
  ProviderFieldsInput,
  ProviderModelFields,
  ProviderModelFieldsInput,
  ProviderModels,
  ProviderHeader,
  ProviderInputRules,
  ProviderInputRulesInput,
} from '../../libs/contract/dist/provider.js'
import { EmailCodeInput, EmailInput } from '../../libs/contract/dist/validation.js'
import { otpCharacters, otpLength, otpRequestSchema } from '../../libs/contract/dist/auth.js'
import { headersAreValid, acceptsHeaderInput } from '../../libs/contract/dist/provider.js'

const schemas = {
  name: ProviderFields.shape.name,
  baseUrl: ProviderFields.shape.baseUrl,
  apiKey: ProviderFields.shape.apiKey,
  modelName: ProviderModelFields.shape.name,
  path: ProviderModelFields.shape.path,
  price: ProviderModelFields.shape.inputPrice,
  context: ProviderModelFields.shape.maxContextTokens,
  effort: ProviderInputRules.shape.effortValue,
  authorization: ProviderInputRules.shape.authorization,
  headerKey: ProviderHeader.shape.key,
  headerValue: ProviderHeader.shape.value,
  header: ProviderHeader,
}
const cases = JSON.parse(await readFile(new URL('./input-contract-fixtures.json', import.meta.url), 'utf8'))
for (const [index, fixture] of cases.entries()) {
  test(`shared acceptance fixture ${index}: ${fixture.kind}`, () => {
    assert.equal(schemas[fixture.kind].safeParse(fixture.value).success, fixture.valid)
  })
}
test('normalization precedes OTP validation; unknown body fields are rejected', () => {
  assert.equal(EmailCodeInput.parse({ code: '\u0085123456\u0085' }).code, '123456')
  assert.equal(EmailCodeInput.safeParse({ code: '\ufeff123456' }).success, false)
  assert.equal(EmailCodeInput.safeParse({ code: '123456', unknown: true }).success, false)
})
test('header names are case-insensitively unique and UI row identifiers are ignored', () => {
  assert.equal(headersAreValid([{ id: 'a', key: 'X-Key', value: '' }]), true)
  assert.equal(
    headersAreValid([
      { key: 'X-Key', value: '' },
      { key: 'x-key', value: '' },
    ]),
    false,
  )
})

test('model collections require a valid entry and unique names', () => {
  const model = {
    name: 'model',
    path: '/responses',
    maxContextTokens: 1,
    inputPrice: '0',
    outputPrice: '0',
    cachedPrice: '0',
  }
  assert.equal(ProviderModels.safeParse({ entries: [] }).success, false)
  assert.equal(ProviderModels.safeParse({ entries: [model] }).success, true)
  assert.equal(ProviderModels.safeParse({ entries: [model, model] }).success, false)
  assert.equal(ProviderModels.safeParse({ entries: [{ ...model, inputPrice: '-1' }] }).success, false)
})

test('generated entry guards reject invalid characters and preserve clearable fields', () => {
  for (const guard of [
    ProviderModelFieldsInput.name,
    ProviderInputRulesInput.effortValue,
    ProviderFieldsInput.apiKey,
  ]) {
    assert.equal(guard('valid-value_1'), true)
    assert.equal(guard(''), true)
    assert.equal(guard('invalid value'), false)
    assert.equal(guard('value\n'), false)
  }
  assert.equal(ProviderModelFieldsInput.name('a'.repeat(41)), false)
  assert.equal(ProviderInputRulesInput.effortValue('a'.repeat(26)), false)
  assert.equal(ProviderFieldsInput.name('😀'.repeat(120)), true)
  assert.equal(ProviderFieldsInput.name('😀'.repeat(121)), false)
  assert.equal(ProviderFieldsInput.apiKey('a'.repeat(513)), false)
  assert.equal(ProviderModelFieldsInput.path('/custom/messages'), true)
  for (const path of ['/with space', '/query?x', '/fragment#x', '/a.b', '/tool\\path']) {
    assert.equal(ProviderModelFieldsInput.path(path), false)
  }
})

test('generated numeric guards allow partial entry and block characters, precision, and upper bounds', () => {
  for (const price of ['', '.', '0.', '12.12345678', '00000000000000001']) {
    assert.equal(ProviderModelFieldsInput.inputPrice(price), true)
  }
  for (const price of ['0.123456789', '-1', '1e3', '1.2.3', '1\n', '1000000000000']) {
    assert.equal(ProviderModelFieldsInput.inputPrice(price), false)
  }
  assert.equal(ProviderModelFields.shape.inputPrice.safeParse('0.').success, false)
  for (const tokens of ['', '0', '250000', '4294967295']) {
    assert.equal(ProviderModelFieldsInput.maxContextTokens(tokens), true)
  }
  for (const tokens of ['1e3', '1.5', '-1', '12x', '4294967296']) {
    assert.equal(ProviderModelFieldsInput.maxContextTokens(tokens), false)
  }
  assert.equal(ProviderModelFields.shape.maxContextTokens.safeParse(0).success, false)
})

test('header entry guards permit incomplete credentials and reject invalid pasted values', () => {
  for (const value of ['', 'B', 'Bearer', 'Bearer ', 'Bearer abc', 'Basic YTo=']) {
    assert.equal(acceptsHeaderInput({ key: 'Authorization', value }), true)
  }
  for (const value of ['Bearer abc def', 'Bearer abc@', 'Basic ab c', 'Bearer abc\n']) {
    assert.equal(acceptsHeaderInput({ key: 'Authorization', value }), false)
  }
  assert.equal(acceptsHeaderInput({ key: '', value: '' }), true)
  assert.equal(acceptsHeaderInput({ key: 'Bad header', value: '' }), false)
  assert.equal(headersAreValid([{ key: 'Authorization', value: 'Bearer ' }]), false)
})

const emailCases = JSON.parse(await readFile(new URL('./auth-input-fixtures.json', import.meta.url), 'utf8'))
for (const [index, fixture] of emailCases.entries()) {
  test(`Rust email policy fixture ${index}: signup and signin`, () => {
    for (const schema of [EmailInput]) {
      assert.equal(schema.safeParse({ email: fixture.value }).success, fixture.valid)
    }
  })
}
test('authentication contracts normalize email and supply OTP entry constraints', () => {
  for (const schema of [EmailInput]) {
    assert.equal(schema.parse({ email: '\u0085DÖRTE@EXAMPLE.COM\u0085' }).email, 'dÖrte@example.com')
    assert.equal(schema.safeParse({ email: 'reader@example.com', unknown: true }).success, false)
  }
  assert.equal(otpLength, 6)
  assert.equal(otpCharacters('12x３4!56'), '12456')
  assert.equal(otpCharacters('1234567'), '123456')
  for (const mode of ['signup', 'signin']) {
    assert.equal(otpRequestSchema(mode).safeParse({ code: '12345' }).success, false)
    assert.equal(otpRequestSchema(mode).parse({ code: '\u0085123456\u0085' }).code, '123456')
  }
})
