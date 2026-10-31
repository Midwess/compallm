import { expect, test } from '@playwright/test'
import { createSignupStore } from '../src/pages/signup'
import { createSigninStore } from '../src/pages/signin'
import { DemoAuthController } from '../stories/pages/demo-auth-controller'

test('page store factories apply overrides and keep page instances isolated', () => {
  const first = createSignupStore({ email: 'alex@example.com' })
  const second = createSignupStore()
  const signin = createSigninStore()
  expect(first.getState()).toMatchObject({ email: 'alex@example.com', pending: null, remaining: 0, step: 'email' })
  first.setState({ email: 'changed@example.com', error: 'invalid-email' })
  expect(second.getState()).toMatchObject({ email: '', error: null })
  expect(signin.getState()).toEqual(second.getState())
  expect(first.getInitialState().email).toBe('alex@example.com')
})

test('controller normalizes email, rejects duplicate pending requests, and guards stopped work', async () => {
  const controller = new DemoAuthController('signup')
  try {
    controller.onEmailChanged(' Alex@Example.com ')
    const operation = controller.requestCode()
    expect(controller.store.getState()).toMatchObject({ email: 'alex@example.com', pending: 'request' })
    await controller.requestCode()
    controller.onEmailChanged('ignored@example.com')
    expect(controller.store.getState().email).toBe('alex@example.com')
    controller.stop()
    await operation
    expect(controller.store.getState()).toMatchObject({ step: 'email', pending: null })
  } finally {
    controller.stop()
  }
})

test('controller retains failed verification and ignores an unchanged failed code', async () => {
  const controller = new DemoAuthController('signin', 'default', { step: 'code', email: 'alex@example.com' })
  try {
    controller.onCodeChanged('654321')
    await expect.poll(() => controller.store.getState().pending).toBeNull()
    expect(controller.store.getState()).toMatchObject({ code: '654321', error: 'incorrect-code', step: 'code' })
    controller.onCodeChanged('654321')
    expect(controller.store.getState()).toMatchObject({ pending: null, error: 'incorrect-code' })
    controller.onCodeChanged('123456')
    await expect.poll(() => controller.store.getState().step).toBe('complete')
  } finally {
    controller.stop()
  }
})

test('class controller stores update only the UI parts selecting the changed values', async ({ page }) => {
  await page.goto('/iframe.html?id=pages-signup--subscriptions&viewMode=story')
  const count = async (id: string) => Number(await page.getByRole('status', { name: `${id} renders` }).textContent())
  await expect(page.getByRole('textbox', { name: 'Email address' })).toBeVisible()
  const before = {
    email: await count('email'),
    continue: await count('continue'),
    code: await count('code'),
    resend: await count('resend'),
  }
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await expect.poll(() => count('email')).toBeGreaterThan(before.email)
  expect(await count('continue')).toBe(before.continue)
  expect(await count('code')).toBe(before.code)
  expect(await count('resend')).toBe(before.resend)
  const emailBeforeTick = await count('email')
  await page.getByRole('button', { name: 'Advance countdown' }).click()
  await expect.poll(() => count('resend')).toBeGreaterThan(before.resend)
  expect(await count('email')).toBe(emailBeforeTick)
  expect(await count('code')).toBe(before.code)
})
