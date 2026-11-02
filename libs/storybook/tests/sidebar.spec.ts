import { expect, test } from '@playwright/test'

const story = '/iframe.html?id=components-navigation-controls-sidebar--'

test('sidebar groups and nested models collapse independently', async ({ page }) => {
  await page.goto(`${story}expanded&viewMode=story`)
  const nav = page.getByRole('navigation', { name: 'Workspace navigation' })
  await expect(nav.getByRole('link', { name: 'Fast Chat', exact: true })).toBeVisible()
  await nav.locator('summary').filter({ hasText: 'Models' }).click()
  await expect(nav.getByRole('link', { name: 'Fast Chat', exact: true })).not.toBeVisible()
  await expect(nav.getByRole('link', { name: 'Stats', exact: true })).toBeVisible()
  await nav.locator('summary').filter({ hasText: 'Providers' }).focus()
  await page.keyboard.press('Enter')
  await expect(nav.getByRole('link', { name: 'OpenAI', exact: true })).not.toBeVisible()
  await expect(nav.getByRole('link', { name: 'Members', exact: true })).toBeVisible()
})

test('sidebar actions and navigation are caller-owned', async ({ page }) => {
  await page.goto(`${story}expanded&viewMode=story`)
  const members = page.getByRole('link', { name: 'Members', exact: true })
  await members.click()
  await expect(members).toHaveAttribute('aria-current', 'page')
  await page.getByRole('button', { name: 'Add model', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Sidebar action' })).toHaveText('Add model')
  await expect(page.getByRole('link', { name: 'Fast Chat', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Search workspace', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Sidebar action' })).toHaveText('Workspace search')
})

test('collapsed sidebar retains its header and can open a section', async ({ page }) => {
  await page.goto(`${story}collapsed&viewMode=story`)
  await expect(page.getByRole('button', { name: 'Switch workspace: Compallm', exact: true })).toBeVisible()
  await expect(page.getByRole('link', { name: 'Members', exact: true })).not.toBeVisible()
  await page.locator('summary').filter({ hasText: 'Workspace' }).click()
  await expect(page.getByRole('link', { name: 'Members', exact: true })).toBeVisible()
  const sidebar = page.locator('aside')
  expect((await sidebar.boundingBox())!.width).toBe(244)
})

test('expanded sidebar preserves the Penpot geometry', async ({ page }) => {
  await page.goto(`${story}expanded&viewMode=story`)
  const sidebar = page.locator('aside')
  await expect(sidebar).toHaveCSS('width', '244px')
  await expect(sidebar).toHaveCSS('padding', '4px 12px')
  for (const [name, top] of [
    ['OpenAI', 104],
    ['Stats', 230],
    ['Fast Chat', 320],
    ['Members', 416],
    ['Exploring a new idea', 542],
  ] as const) {
    const row = await sidebar.getByRole('link', { name, exact: true }).boundingBox()
    expect(row!.height).toBe(28)
    expect(row!.y).toBe(top)
  }
})

test('workspace menu opens Settings and restores focus on dismissal', async ({ page }) => {
  await page.goto(`${story}expanded&viewMode=story`)
  const trigger = page.getByRole('button', { name: 'Switch workspace: Compallm', exact: true })
  await trigger.click()
  const menu = page.getByRole('menu', { name: 'Workspace options' })
  await expect(menu).toBeVisible()
  await expect(trigger).toHaveAttribute('aria-expanded', 'true')
  await page.getByRole('menuitem', { name: 'Settings', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Sidebar action' })).toHaveText('Settings')
  await expect(menu).not.toBeVisible()
  await expect(trigger).toBeFocused()
  await page.keyboard.press('ArrowDown')
  await expect(menu).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(menu).not.toBeVisible()
  await expect(trigger).toBeFocused()
})

test('organization submenu opens to the right on hover and selects an organization', async ({ page }) => {
  await page.goto(`${story}expanded&viewMode=story`)
  await page.getByRole('button', { name: 'Switch workspace: Compallm', exact: true }).click()
  const switcher = page.getByRole('menuitem', { name: 'Switch organization', exact: true })
  await switcher.hover()
  const submenu = page.getByRole('menu', { name: 'Organizations', exact: true })
  await expect(submenu).toBeVisible()
  await page.waitForTimeout(300)
  const parentBox = await switcher.boundingBox()
  const submenuBox = await submenu.boundingBox()
  expect(submenuBox!.x).toBeGreaterThanOrEqual(parentBox!.x + parentBox!.width - 1)
  await page.getByRole('menuitemradio', { name: 'Research team', exact: true }).click()
  await expect(submenu).not.toBeVisible()
  await expect(page.getByRole('menu', { name: 'Workspace options', exact: true })).not.toBeVisible()
  await expect(page.getByRole('button', { name: 'Switch workspace: Research team', exact: true })).toBeVisible()
})

test('popup scale animation uses 150ms and respects reduced motion', async ({ page }) => {
  await page.goto(`${story}expanded&viewMode=story`)
  const trigger = page.getByRole('button', { name: 'Switch workspace: Compallm', exact: true })
  await trigger.click()
  const menu = page.getByRole('menu', { name: 'Workspace options', exact: true })
  await expect(menu).toHaveCSS('animation-duration', '0.15s')
  await page.keyboard.press('Escape')
  await page.emulateMedia({ reducedMotion: 'reduce' })
  await trigger.click()
  await expect(menu).toHaveCSS('animation-name', 'none')
  await expect(page.getByRole('menuitem', { name: 'Settings', exact: true })).toBeFocused()
  await page.keyboard.press('ArrowDown')
  await page.keyboard.press('ArrowRight')
  await expect(page.getByRole('menu', { name: 'Organizations', exact: true })).toBeVisible()
  await page.keyboard.press('ArrowDown')
  await page.keyboard.press('Enter')
  await expect(page.getByRole('button', { name: 'Switch workspace: Research team', exact: true })).toBeVisible()
})
