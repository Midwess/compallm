import { expect, test } from '@playwright/test'

const preview = '/iframe.html?id=pages-organization-chat--new-chat&viewMode=story'

test('create-chat page has only New chat and a centered input', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 })
  await page.goto(preview)
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  await expect(page.getByRole('region', { name: 'Conversation messages' })).toHaveCount(0)
  const content = await page.getByRole('main', { name: 'Workspace content' }).boundingBox()
  const form = await page.getByRole('form', { name: 'Create chat', exact: true }).boundingBox()
  expect(Math.abs(form!.x + form!.width / 2 - content!.x - content!.width / 2)).toBeLessThan(1)
  expect(Math.abs(form!.y + form!.height / 2 - content!.y - content!.height / 2)).toBeLessThan(30)
})

test('create-chat inputs only notify the controller without navigating or calling APIs', async ({ page }) => {
  const calls: string[] = []
  page.on('request', (request) => {
    if (new URL(request.url()).pathname.startsWith('/api/')) calls.push(request.url())
  })
  await page.goto(preview)
  const create = page.getByRole('button', { name: 'Create chat', exact: true })
  await expect(create).toBeDisabled()
  const draft = page.getByRole('textbox', { name: 'Message', exact: true })
  await draft.fill('Help me explore an idea.')
  await draft.press('Shift+Enter')
  await expect(draft).toHaveValue('Help me explore an idea.\n')
  await draft.press('Enter')
  await expect(draft).toHaveValue('Help me explore an idea.\n\n')
  await expect(page.getByRole('status', { name: 'Chat intent' })).toBeEmpty()
  await create.click()
  await expect(page.getByRole('status', { name: 'Chat intent' })).toHaveText('Create chat requested')
  expect(page.url()).toContain('id=pages-organization-chat--new-chat')
  expect(calls).toEqual([])
})

test('multiline input grows, caps its height, scrolls, and shrinks after clearing', async ({ page }) => {
  await page.goto(preview)
  const draft = page.getByRole('textbox', { name: 'Message', exact: true })
  const initial = (await draft.boundingBox())!.height
  await draft.fill(Array.from({ length: 6 }, (_, i) => `Line ${i}`).join('\n'))
  expect((await draft.boundingBox())!.height).toBeGreaterThan(initial)
  await draft.fill(Array.from({ length: 40 }, (_, i) => `Line ${i}`).join('\n'))
  expect((await draft.boundingBox())!.height).toBeLessThanOrEqual(256)
  expect(await draft.evaluate((element) => element.scrollHeight > element.clientHeight)).toBe(true)
  await draft.fill('')
  expect((await draft.boundingBox())!.height).toBe(initial)
  const create = page.getByRole('button', { name: 'Create chat', exact: true })
  await expect(create).toBeDisabled()
  const shadow = await create.evaluate((element) => getComputedStyle(element).boxShadow)
  const invisible = 'rgba(0, 0, 0, 0) 0px 0px 0px 0px'
  expect(shadow === 'none' || shadow.split(invisible).join('').replaceAll(',', '').trim() === '').toBe(true)
  await expect(create).toHaveCSS('border-color', 'rgba(0, 0, 0, 0)')
})

test('creating and missing-model states prevent chat creation', async ({ page }) => {
  await page.goto('/iframe.html?id=pages-organization-chat--creating&viewMode=story')
  await expect(page.getByRole('textbox', { name: 'Message', exact: true })).toBeDisabled()
  await expect(page.getByRole('button', { name: 'Create chat', exact: true })).toBeDisabled()
  await page.goto('/iframe.html?id=pages-organization-chat--no-models&viewMode=story')
  await page.getByRole('textbox', { name: 'Message', exact: true }).fill('Hello')
  await expect(page.getByRole('button', { name: 'Create chat', exact: true })).toBeDisabled()
})
