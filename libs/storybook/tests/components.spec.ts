import { expect, test } from '@playwright/test'

test('every base preview renders inside Components without backend requests', async ({ page, request }) => {
  const response = await request.get('/index.json')
  expect(response.ok()).toBe(true)
  const index = (await response.json()) as { entries: Record<string, { id: string; title: string; type: string }> }
  const stories = Object.values(index.entries).filter(
    (entry) => entry.type === 'story' && entry.title.startsWith('Components/'),
  )
  expect(stories.length).toBeGreaterThanOrEqual(74)
  expect(stories.every((story) => story.title.startsWith('Components/'))).toBe(true)
  const errors: string[] = []
  const backendRequests: string[] = []
  page.on('pageerror', (error) => errors.push(error.message))
  page.on('request', (request) => {
    if (new URL(request.url()).pathname.startsWith('/api/')) backendRequests.push(request.url())
  })
  for (const story of stories) {
    await page.goto(`/iframe.html?id=${story.id}&viewMode=story`)
    await expect(page.locator('#storybook-root [data-ui="component-preview"]'), story.id).toBeVisible()
    await expect(page.locator('#storybook-root')).not.toBeEmpty()
  }
  expect(errors).toEqual([])
  expect(backendRequests).toEqual([])
})

test('dropdown selects with a keyboard, skips disabled options, and restores focus', async ({ page }) => {
  await page.goto('/iframe.html?id=components-dropdowns--dropdown&viewMode=story')
  const trigger = page.getByRole('button', { name: 'Language', exact: true })
  await trigger.focus()
  await page.keyboard.press('ArrowDown')
  await expect(page.getByRole('listbox', { name: 'Language', exact: true })).toBeFocused()
  await page.keyboard.press('ArrowDown')
  await page.keyboard.press('Enter')
  await expect(page.getByRole('status', { name: 'Selected language' })).toHaveText('rust')
  await expect(trigger).toBeFocused()
  await trigger.click()
  await page.keyboard.press('End')
  await page.keyboard.press('Enter')
  await expect(page.getByRole('status', { name: 'Selected language' })).toHaveText('python')
  await trigger.click()
  await page.keyboard.press('Escape')
  await expect(trigger).toBeFocused()
  await expect(trigger).toHaveAttribute('aria-expanded', 'false')
})

test('select uses the themed floating menu without resizing its preview', async ({ page }) => {
  await page.goto('/iframe.html?id=components-dropdowns--select&viewMode=story')
  const trigger = page.getByRole('button', { name: 'Language', exact: true })
  const preview = page.locator('[data-ui="component-preview"]')
  await expect(trigger).toBeVisible()
  await page.evaluate(() => document.fonts.ready)
  const before = await preview.boundingBox()
  const control = await trigger.boundingBox()
  await trigger.click()
  const menu = page.getByRole('listbox', { name: 'Language', exact: true })
  await expect(menu).toBeVisible()
  expect(await menu.evaluate((element) => element.matches(':popover-open'))).toBe(true)
  expect(await page.locator('select').count()).toBe(0)
  expect(await preview.boundingBox()).toEqual(before)
  const panel = await menu.boundingBox()
  expect(panel!.y).toBeGreaterThanOrEqual(control!.y + control!.height)
  expect(panel!.x).toBeCloseTo(control!.x, 0)
  expect(panel!.width).toBeCloseTo(control!.width, 0)
  await page.getByRole('option', { name: 'Rust', exact: true }).click()
  await expect(trigger).toHaveText('Rust')
  await expect(trigger).toBeFocused()
  await trigger.click()
  await page.getByText('Custom select', { exact: true }).click()
  await expect(menu).not.toBeVisible()
  await trigger.evaluate((element) => {
    element.style.position = 'fixed'
    element.style.right = '8px'
    element.style.bottom = '8px'
    element.style.width = '240px'
  })
  await trigger.click()
  await expect(menu).toBeVisible()
  const edgeMenu = await menu.boundingBox()
  const edgeTrigger = await trigger.boundingBox()
  expect(edgeMenu!.y + edgeMenu!.height).toBeLessThanOrEqual(edgeTrigger!.y)
  expect(edgeMenu!.x).toBeGreaterThanOrEqual(0)
  expect(edgeMenu!.x + edgeMenu!.width).toBeLessThanOrEqual(page.viewportSize()!.width)
})

test('dropdown docs keep preview containers stable when a menu opens', async ({ page }, testInfo) => {
  await page.goto('/?path=/docs/components-dropdowns--docs')
  const docs = page.frameLocator('#storybook-preview-iframe')
  const preview = docs.locator('#story--components-dropdowns--select [data-ui="component-preview"]')
  await expect(preview).toBeVisible()
  await preview.evaluate(() => document.fonts.ready)
  const container = preview.locator('..')
  const before = await container.boundingBox()
  const trigger = preview.getByRole('button', { name: 'Language', exact: true })
  await trigger.click()
  const menu = docs.getByRole('listbox', { name: 'Language', exact: true })
  await expect(menu).toBeVisible()
  expect(await container.boundingBox()).toEqual(before)
  await page.screenshot({ path: testInfo.outputPath('dropdown-docs-open.png') })
  await page.keyboard.press('Escape')
  await expect(menu).not.toBeVisible()
  expect(await container.boundingBox()).toEqual(before)
})

test('code entry keeps numeric input and field errors are associated', async ({ page }) => {
  await page.goto('/iframe.html?id=components-text-fields--code-entry&viewMode=story')
  const group = page.getByRole('group', { name: 'Verification code', exact: true })
  const digits = group.getByRole('textbox')
  await digits.first().fill('12a3b4')
  await expect(digits.nth(0)).toHaveValue('1')
  await expect(digits.nth(3)).toHaveValue('4')
  await digits.first().fill('987654')
  await expect(digits.last()).toHaveValue('4')
  await expect(page.getByRole('group', { name: 'Disabled code', exact: true }).getByRole('textbox').first()).toBeDisabled()
  await page.goto('/iframe.html?id=components-form-primitives--labeled-field&viewMode=story')
  const invalid = page.getByRole('textbox', { name: 'Invalid application name', exact: true })
  await expect(invalid).toHaveAttribute('aria-invalid', 'true')
  await expect(invalid).toHaveAccessibleDescription('Use a recognizable name. A name is required.')
})

test('popover and tooltip can be dismissed from the keyboard', async ({ page }) => {
  await page.goto('/iframe.html?id=components-feedback-overlays--popover&viewMode=story')
  const trigger = page.getByRole('button', { name: 'Display options' })
  await trigger.click()
  await expect(page.getByText('Show descriptions', { exact: true })).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(page.getByText('Show descriptions', { exact: true })).not.toBeVisible()
  await expect(trigger).toBeFocused()
  await page.goto('/iframe.html?id=components-feedback-overlays--tooltip&viewMode=story')
  await page.getByRole('button', { name: 'Add provider' }).focus()
  await expect(page.getByRole('tooltip')).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(page.getByRole('tooltip')).not.toBeVisible()
})

test('text fields and project tables fit a narrow preview', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 })
  for (const id of [
    'components-text-fields--text-field',
    'components-text-fields--code-entry',
    'components-data-primitives--project-table-row',
  ]) {
    await page.goto(`/iframe.html?id=${id}&viewMode=story`)
    await expect(page.locator('[data-ui="component-preview"]')).toBeVisible()
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth), id).toBe(true)
  }
})
