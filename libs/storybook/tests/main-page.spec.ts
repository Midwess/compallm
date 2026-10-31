import { expect, test } from '@playwright/test'

test('main shell matches the Penpot content bounds', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 })
  await page.goto('/iframe.html?id=pages-main--default&viewMode=story')
  const content = page.getByRole('main', { name: 'Workspace content' })
  await expect(content).toBeVisible()
  const box = await content.boundingBox()
  expect(box).toMatchObject({ x: 244, y: 16, width: 1180, height: 952 })
  await expect(content).toBeEmpty()
  await expect(page.getByRole('link', { name: 'Members', exact: true })).toBeVisible()
})

test('feature content scrolls without moving sidebar navigation', async ({ page }) => {
  await page.setViewportSize({ width: 1000, height: 640 })
  await page.goto('/iframe.html?id=pages-main--scrollable-content&viewMode=story')
  const sidebar = page.locator('aside')
  await expect(sidebar).toBeVisible()
  const before = await sidebar.boundingBox()
  const main = page.getByRole('main', { name: 'Workspace content' })
  await main.evaluate((element) => {
    element.scrollTop = element.scrollHeight
  })
  await expect(page.getByText('Example item 60', { exact: true })).toBeInViewport()
  expect(await sidebar.boundingBox()).toEqual(before)
  expect(await page.evaluate(() => document.documentElement.scrollHeight)).toBe(640)
})
