import { expect, test } from '@playwright/test'
const user = { userId: 42, organizationId: 84, organizationName: 'Research team', sessionId: 7, organizationRole: 'owner' }

test('sidebar Add provider soft-navigates into the organization-scoped provider page', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  let documents = 0
  page.on('request', (request) => {
    if (request.isNavigationRequest()) documents += 1
  })
  await page.goto('/Research%20team/chat')
  await page.getByRole('button', { name: 'Add provider', exact: true }).click()
  await expect(page).toHaveURL('/Research%20team/providers/new/choose-provider')
  await expect(page.getByRole('searchbox', { name: 'Search providers…' })).toBeVisible()
  await page.getByRole('button', { name: 'Choose OpenAI', exact: true }).click()
  await expect(page.getByLabel('Base URL', { exact: true })).toHaveValue('https://api.openai.com/v1')
  await expect(page.getByRole('button', { name: 'Switch workspace: Research team', exact: true })).toBeVisible()
  expect(documents).toBe(1)
})

test('direct anonymous provider visits require signin', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.goto('/Research%20team/providers/new')
  await expect(page).toHaveURL('/login')
  await expect(page.getByRole('searchbox', { name: 'Search providers…' })).toHaveCount(0)
})

test('portal edits custom models but does not simulate provider persistence or upstream tests', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  const providerCalls: string[] = []
  page.on('request', (request) => {
    if (request.url().includes('/api/providers')) providerCalls.push(request.url())
  })
  await page.goto('/Research%20team/providers/new')
  await page.getByRole('button', { name: 'Use custom provider', exact: true }).click()
  await page.getByLabel('Connection name', { exact: true }).fill('Internal inference')
  await page.getByLabel('Base URL', { exact: true }).fill('https://inference.example.com/v1')
  await page.getByLabel('API key *', { exact: true }).fill('synthetic-test-key')
  await page.getByRole('button', { name: 'Add custom model', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await dialog.getByLabel('Model name', { exact: true }).fill('internal-model')
  for (const label of ['1M Input', '1M Output', '1M Cached']) await dialog.getByLabel(label, { exact: true }).fill('0')
  await dialog.getByRole('button', { name: 'Add model', exact: true }).click()
  await expect(page.getByRole('checkbox', { name: 'Enable internal-model', exact: true })).toBeChecked()
  await page.getByRole('button', { name: 'Test connection', exact: true }).click()
  await expect(page.getByRole('status').filter({ hasText: 'Connection testing is not available yet.' })).toBeVisible()
  await page.getByRole('main').getByRole('button', { name: 'Add provider', exact: true }).click()
  await expect(
    page
      .getByRole('status')
      .filter({ hasText: 'Provider creation is not available yet. Your configuration has not been saved.' }),
  ).toBeVisible()
  expect(providerCalls).toEqual([])
  await expect(page.getByText('Check passed', { exact: true })).toHaveCount(0)
})

test('provider connection and model dialog state restore from URL after reload without secrets', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/providers/new/choose-provider')
  await page.getByRole('button', { name: 'Choose OpenAI', exact: true }).click()
  await expect(page).toHaveURL(/new\/configure-provider\?/)
  await page.getByLabel('Connection name', { exact: true }).fill('My production')
  await page.getByLabel('API key *', { exact: true }).fill('never-put-this-key-in-url')
  await page.getByRole('button', { name: 'Add custom model', exact: true }).click()
  let dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await dialog.getByLabel('Model name', { exact: true }).fill('private-model')
  await dialog.getByLabel('1M Input', { exact: true }).fill('0.25')
  await expect.poll(() => new URL(page.url()).searchParams.get('input_price')).toBe('0.25')
  expect(page.url()).not.toContain('never-put-this-key-in-url')
  page.on('dialog', (dialog) => dialog.accept())
  await page.reload()
  dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await expect(dialog.getByLabel('Model name', { exact: true })).toHaveValue('private-model')
  await expect(dialog.getByLabel('1M Input', { exact: true })).toHaveValue('0.25')
  await expect(page.getByLabel('Connection name', { exact: true })).toHaveValue('My production')
  await expect(page.getByLabel('API key *', { exact: true })).toHaveValue('')
})

test('configured model prices and search filters survive reload', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/providers/new/configure-provider?provider=custom&name=Internal')
  await page.getByRole('button', { name: 'Add custom model', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await dialog.getByLabel('Model name', { exact: true }).fill('custom-model')
  for (const label of ['1M Input', '1M Output', '1M Cached']) await dialog.getByLabel(label, { exact: true }).fill('0')
  await dialog.getByRole('button', { name: 'Add model', exact: true }).click()
  await page.getByRole('searchbox', { name: 'Search models…' }).fill('custom')
  await expect.poll(() => new URL(page.url()).searchParams.get('model_q')).toBe('custom')
  await page.reload()
  await expect(page.getByRole('checkbox', { name: 'Enable custom-model', exact: true })).toBeChecked()
  await expect(page.getByRole('searchbox', { name: 'Search models…' })).toHaveValue('custom')
  await expect(page.getByRole('row').filter({ hasText: 'custom-model' })).toContainText('$0')
})

test('Back and Forward restore workflow steps and prompt before discarding a model draft', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/providers/new/choose-provider')
  await page.getByRole('button', { name: 'Choose OpenAI', exact: true }).click()
  await page.getByRole('button', { name: 'Add custom model', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await dialog.getByLabel('Model name', { exact: true }).fill('keep-me')
  await expect.poll(() => new URL(page.url()).searchParams.get('model_name')).toBe('keep-me')
  await page.evaluate(() => history.back())
  const warning = page.getByRole('alertdialog', { name: 'Discard changes?', exact: true })
  await expect(warning).toBeVisible()
  await warning.getByRole('button', { name: 'Keep editing', exact: true }).click()
  await expect(dialog.getByLabel('Model name', { exact: true })).toHaveValue('keep-me')
  await page.evaluate(() => history.back())
  await warning.getByRole('button', { name: 'Discard changes', exact: true }).click()
  await expect(dialog).not.toBeVisible()
  await page.goBack()
  await expect(page.getByRole('searchbox', { name: 'Search providers…' })).toBeVisible()
  await page.goForward()
  await expect(page.getByLabel('Connection name', { exact: true })).toHaveValue('OpenAI production')
})

test('invalid model URLs fail safely instead of restoring a fake successful configuration', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/providers/new/configure-provider?provider=openai&models=invalid&v=1')
  await expect(page.getByRole('alert')).toContainText('The model data in this URL is invalid.')
  await expect(page.getByRole('main').getByRole('button', { name: 'Add provider', exact: true })).toBeDisabled()
})

test('rapid URL updates merge and typing does not add history entries or repeat identity checks', async ({ page }) => {
  let checks = 0
  await page.route('**/api/auth/me', (route) => {
    checks += 1
    return route.fulfill({ json: user })
  })
  await page.goto('/Research%20team/providers/new/choose-provider')
  await page.getByRole('button', { name: 'Choose OpenAI', exact: true }).click()
  const name = page.getByLabel('Connection name', { exact: true })
  await name.fill('')
  await name.pressSequentially('Restored name')
  await page.evaluate(async () => {
    const { urlController } = await import('/src/url/controller.ts')
    urlController.update({ q: 'openai' })
    urlController.update({ model_q: 'custom' })
  })
  await expect.poll(() => new URL(page.url()).searchParams.get('q')).toBe('openai')
  await expect.poll(() => new URL(page.url()).searchParams.get('model_q')).toBe('custom')
  expect(new URL(page.url()).searchParams.get('name')).toBe('Restored name')
  expect(checks).toBe(1)
  await page.goBack()
  await expect(page.getByRole('searchbox', { name: 'Search providers…' })).toBeVisible()
})

test('URL limits reject oversized drafts and credential parameters are removed on entry', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/providers/new/configure-provider?provider=custom&api_key=secret')
  await expect.poll(() => new URL(page.url()).searchParams.has('api_key')).toBe(false)
  await page.evaluate(async () => {
    const { urlController } = await import('/src/url/controller.ts')
    urlController.update({ name: 'a'.repeat(20000) })
  })
  await expect(page.getByRole('status').filter({ hasText: 'This draft is too large to store in the URL.' })).toBeVisible()
  expect(page.url().length).toBeLessThan(16384)
})
