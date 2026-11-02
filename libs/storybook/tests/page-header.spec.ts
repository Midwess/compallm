import { expect, test } from '@playwright/test'

test('page header aligns title with body content and invokes supplied actions', async ({ page }) => {
  await page.goto('/iframe.html?id=components-navigation-controls-page-header--default&viewMode=story')
  const title = page.getByRole('heading', { name: 'Members', exact: true })
  const body = page.locator('[data-ui="content-start"]')
  await expect(title).toHaveCSS('font-size', '12px')
  await expect(page.locator('[data-ui=page-header]')).toHaveCSS('padding-left', '12px')
  await expect(page.locator('[data-ui=page-header]')).toHaveCSS('padding-right', '12px')
  const a = await title.boundingBox()
  const b = await body.boundingBox()
  expect(a!.x).toBe(b!.x)
  await page.getByRole('button', { name: 'Invite member', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Header action' })).toHaveText('Invite member')
})

test('long header titles truncate and keep actions inside the viewport', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 640 })
  await page.goto('/iframe.html?id=components-navigation-controls-page-header--long-title&viewMode=story')
  const action = page.getByRole('button', { name: 'Invite member', exact: true })
  await expect(action).toBeVisible()
  const bounds = await action.boundingBox()
  expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(375)
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
})
