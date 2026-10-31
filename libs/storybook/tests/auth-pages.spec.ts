import { expect, test } from '@playwright/test'

test('signup and signin have clean preview routes with working account links', async ({ page }, testInfo) => {
  const backendRequests: string[] = []
  page.on('request', (request) => {
    if (new URL(request.url()).pathname.startsWith('/api/')) backendRequests.push(request.url())
  })
  await page.goto('/signup')
  await expect(page.getByRole('heading', { name: 'Create your account.' })).toBeVisible()
  await page.evaluate(() => document.fonts.ready)
  await page.screenshot({ path: testInfo.outputPath('signup.png') })
  await page.getByRole('link', { name: 'Sign in', exact: true }).click()
  await expect(page).toHaveURL(/\/signin$/)
  await expect(page.getByRole('heading', { name: 'Welcome back.' })).toBeVisible()
  await page.screenshot({ path: testInfo.outputPath('signin.png') })
  await page.getByRole('link', { name: 'Sign up', exact: true }).click()
  await expect(page).toHaveURL(/\/signup$/)
  expect(backendRequests).toEqual([])
})

test('signup requests a code, verifies automatically, and completes the demo', async ({ page }) => {
  await page.goto('/signup')
  await page.getByRole('textbox', { name: 'Email address' }).fill('Alex@Example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Verify your email.' })).toBeVisible()
  await expect(page.getByText('alex@example.com', { exact: true })).toBeVisible()
  const digits = page.getByRole('group', { name: 'Email code' }).getByRole('textbox')
  await expect(digits.first()).toBeFocused()
  await page.keyboard.type('123456')
  await expect(page.getByRole('heading', { name: 'Your account is ready.' })).toBeVisible()
})

test('existing signup email stays on the email step and offers signin', async ({ page }) => {
  await page.goto('/signup')
  await page.getByRole('textbox', { name: 'Email address' }).fill('registered@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('alert')).toHaveText('Email already registered. Sign in instead.')
  await expect(page.getByRole('textbox', { name: 'Email address' })).toHaveValue('registered@example.com')
  await expect(page.getByRole('group', { name: 'Email code' })).toHaveCount(0)
})

test('incorrect code is retained, backspace navigates, and a corrected code verifies', async ({ page }) => {
  await page.goto('/iframe.html?id=pages-signin--check-email&viewMode=story')
  const digits = page.getByRole('group', { name: 'Email code' }).getByRole('textbox')
  await digits.first().fill('654321')
  await expect(page.getByRole('alert')).toContainText('Incorrect code')
  await expect(digits.last()).toHaveValue('1')
  await digits.last().fill('')
  await page.keyboard.press('Backspace')
  await expect(digits.nth(4)).toBeFocused()
  await digits.first().fill('123456')
  await expect(page.getByRole('heading', { name: 'You’re signed in.' })).toBeVisible()
})

test('resend is explicit and a temporary verification failure can be retried', async ({ page }) => {
  await page.goto('/iframe.html?id=pages-signup--verify-email&viewMode=story')
  await page.getByRole('button', { name: 'Didn’t receive a code?' }).click()
  await expect(page.getByRole('button', { name: 'Resend code', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Resend code', exact: true }).click()
  await expect(page.getByRole('button', { name: /Resend in/ })).toBeDisabled()
  await page.goto('/iframe.html?id=pages-signin--temporary-failure&viewMode=story')
  await page.getByRole('group', { name: 'Email code' }).getByRole('textbox').first().fill('123456')
  await expect(page.getByRole('alert')).toContainText('Something went wrong')
  await page.getByRole('button', { name: 'Try again', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'You’re signed in.' })).toBeVisible()
})

test('auth pages keep the form usable at a narrow width', async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 375, height: 812 })
  await page.goto('/signup')
  await expect(page.getByRole('textbox', { name: 'Email address' })).toBeVisible()
  await expect(page.locator('[data-ui="auth-introduction"]')).not.toBeVisible()
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
  await page.screenshot({ path: testInfo.outputPath('signup-mobile.png') })
})

test('pending page demos disable duplicate sends and code verification', async ({ page }) => {
  await page.goto('/iframe.html?id=pages-signup--sending-code&viewMode=story')
  await expect(page.getByRole('button', { name: 'Sending code…' })).toBeDisabled()
  await expect(page.getByRole('textbox', { name: 'Email address' })).toBeDisabled()
  await page.goto('/iframe.html?id=pages-signin--checking-code&viewMode=story')
  await expect(page.getByRole('status')).toHaveText('Checking code…')
  await expect(page.getByRole('group', { name: 'Email code' }).getByRole('textbox').first()).toBeDisabled()
})
