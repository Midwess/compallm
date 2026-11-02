import { expect, test } from '@playwright/test'

const preview = '/iframe.html?id=pages-organization--default&viewMode=story'

test('organization page notifies its controller without navigating or calling APIs', async ({ page }) => {
  const calls: string[] = []
  page.on('request', (request) => {
    if (new URL(request.url()).pathname.startsWith('/api/')) calls.push(request.url())
  })
  await page.goto(preview)
  await expect(page.getByRole('main', { name: 'Workspace content' })).toBeEmpty()
  await page.getByRole('button', { name: 'New conversation', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Controller intent' })).toHaveText('Open chat: new → /chat')
  await page.getByRole('link', { name: 'Exploring a new idea', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Controller intent' })).toHaveText('Open chat: exploring → /chat')
  expect(page.url()).toContain('id=pages-organization--default')
  expect(calls).toEqual([])
})

test('organization selection and Settings notify the supplied controller', async ({ page }) => {
  await page.goto(preview)
  await page.getByRole('button', { name: 'Switch workspace: Compallm', exact: true }).click()
  await page.getByRole('menuitem', { name: 'Settings', exact: true }).click()
  await expect(page.getByRole('status', { name: 'Controller intent' })).toHaveText('Open settings')
  await page.getByRole('button', { name: 'Switch workspace: Compallm', exact: true }).click()
  await page.getByRole('menuitem', { name: 'Switch organization', exact: true }).hover()
  await page.getByRole('menuitemradio', { name: 'Research team', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Switch workspace: Research team', exact: true })).toBeVisible()
  await expect(page.getByRole('status', { name: 'Controller intent' })).toHaveText('Switch organization: research')
})
