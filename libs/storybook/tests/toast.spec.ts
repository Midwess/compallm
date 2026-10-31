import { expect, test } from '@playwright/test'

for (const [tone, role, message] of [
  ['info', 'status', 'Your changes have been saved.'],
  ['error', 'alert', 'Couldn’t save changes. Please try again.'],
  ['warning', 'status', 'Your session will expire soon.'],
] as const) {
  test(`${tone} notification announces its message and dismisses with the keyboard`, async ({ page }) => {
    await page.goto(`/iframe.html?id=components-feedback-overlays-toast--${tone}&viewMode=story`)
    await expect(page.getByRole(role)).toContainText(message)
    const show = page.getByRole('button', { name: 'Show notification', exact: true })
    await page.getByRole('button', { name: 'Dismiss notification', exact: true }).focus()
    await page.keyboard.press('Enter')
    await expect(page.locator('[data-ui="toast"]')).toHaveCount(0)
    await show.click()
    await expect(page.getByRole(role)).toContainText(message)
    await expect(show).toBeFocused()
  })
}

test('long notifications wrap within a narrow viewport', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 640 })
  await page.goto('/iframe.html?id=components-feedback-overlays-toast--long-message&viewMode=story')
  await expect(page.getByRole('alert')).toContainText('Your current input has been preserved.')
  await expect(page.getByRole('button', { name: 'Dismiss notification', exact: true })).toBeVisible()
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
})
