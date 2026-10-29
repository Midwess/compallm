import { expect, test } from '@playwright/test'

test('every story renders without browser errors or backend requests', async ({ page, request }) => {
  const response = await request.get('/index.json')
  expect(response.ok()).toBe(true)
  const index = await response.json() as { entries: Record<string, { id: string; type: string }> }
  const errors: string[] = []
  const backendRequests: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  page.on('request', request => {
    if (new URL(request.url()).pathname.startsWith('/api/')) backendRequests.push(request.url())
  })

  for (const story of Object.values(index.entries).filter(entry => entry.type === 'story')) {
    await page.goto(`/iframe.html?id=${story.id}&viewMode=story`)
    await expect(page.locator('#storybook-root .design-system').first()).toBeVisible()
    await expect(page.locator('#storybook-root')).not.toBeEmpty()
  }
  expect(errors).toEqual([])
  expect(backendRequests).toEqual([])
})

test('wireframe navigation works with a keyboard and fits mobile', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 })
  await page.goto('/iframe.html?id=wireframes-workspace--empty&viewMode=story')
  const notebooks = page.getByRole('button', { name: 'Notebooks', exact: true })
  await notebooks.focus()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('heading', { name: 'Notebooks', exact: true })).toBeVisible()
  await expect(notebooks).toHaveAttribute('aria-current', 'page')
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
})
