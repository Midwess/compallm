import { expect, test } from '@playwright/test'
const story = '/iframe.html?id=pages-organization-create-provider--'

test('provider selection filters the catalog and opens its configuration', async ({ page }) => {
  const apiCalls: string[] = []
  page.on('request', (request) => {
    if (new URL(request.url()).pathname.startsWith('/api/')) apiCalls.push(request.url())
  })
  await page.goto(`${story}choose-provider&viewMode=story`)
  await page.getByRole('searchbox', { name: 'Search providers…' }).fill('anthropic')
  await expect(page.getByRole('table', { name: 'Providers', exact: true }).getByRole('row')).toHaveCount(2)
  await page.getByRole('button', { name: 'Anthropic anthropic', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Continue', exact: true })).toHaveCount(0)
  await expect(page.getByLabel('Connection name', { exact: true })).toHaveValue('Anthropic production')
  await expect(page.getByLabel('Base URL', { exact: true })).toHaveValue('https://api.anthropic.com/v1')
  await expect(page.getByRole('button', { name: 'Mode', exact: true })).toHaveText('Messages')
  expect(apiCalls).toEqual([])
})

test('custom models require explicit prices and accept an intentional free rate', async ({ page }) => {
  await page.goto(`${story}custom-provider&viewMode=story`)
  await page.getByRole('button', { name: 'Add custom model', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await dialog.getByLabel('Model name', { exact: true }).fill('my-model')
  await dialog.getByLabel('1M Input', { exact: true }).fill('0.25')
  await dialog.getByLabel('1M Output', { exact: true }).fill('1.00')
  const save = dialog.getByRole('button', { name: 'Add model', exact: true })
  await expect(save).toBeDisabled()
  await dialog.getByLabel('1M Cached', { exact: true }).fill('0')
  await expect(save).toBeEnabled()
  await save.click()
  await expect(dialog).not.toBeVisible()
  const row = page.getByRole('row').filter({ hasText: 'my-model' })
  await expect(row).toContainText('$0.25')
  await expect(row).toContainText('$1.00')
  await expect(row).toContainText('$0')
  await expect(page.getByRole('checkbox', { name: 'Enable my-model', exact: true })).toBeChecked()
})

test('editing one model preserves others and cancellation discards its draft', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }).click()
  let dialog = page.getByRole('dialog', { name: 'Edit model', exact: true })
  await dialog.getByLabel('1M Input', { exact: true }).fill('3.00')
  await dialog.getByRole('button', { name: 'Cancel', exact: true }).click()
  await page
    .getByRole('alertdialog', { name: 'Discard changes?', exact: true })
    .getByRole('button', { name: 'Discard changes', exact: true })
    .click()
  await expect(
    page.getByRole('row').filter({ has: page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }) }),
  ).toContainText('$2.00')
  await page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }).click()
  dialog = page.getByRole('dialog', { name: 'Edit model', exact: true })
  await dialog.getByLabel('1M Input', { exact: true }).fill('3.00')
  await dialog.getByRole('button', { name: 'Save model', exact: true }).click()
  await expect(
    page.getByRole('row').filter({ has: page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }) }),
  ).toContainText('$3.00')
  await expect(
    page.getByRole('row').filter({ has: page.getByRole('button', { name: 'Edit gpt-4.1-mini', exact: true }) }),
  ).toContainText('$0.40')
})

test('models missing required rates and incompatible modes cannot be enabled', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await expect(page.getByRole('checkbox', { name: 'Enable o3', exact: true })).toBeDisabled()
  await expect(page.getByRole('checkbox', { name: 'Enable text-embedding-3-small', exact: true })).toBeDisabled()
  await expect(page.getByRole('checkbox', { name: 'Enable gpt-realtime-1.5', exact: true })).toBeDisabled()
  await page.getByRole('button', { name: 'Mode', exact: true }).click()
  await page.getByRole('option', { name: 'Realtime', exact: true }).click()
  await expect(page.getByRole('checkbox', { name: 'Enable gpt-4.1', exact: true })).not.toBeChecked()
  await expect(page.getByRole('checkbox', { name: 'Enable gpt-4.1', exact: true })).toBeDisabled()
  await expect(
    page.getByRole('button', { name: 'Add provider', exact: true }).filter({ hasNot: page.locator('svg') }),
  ).toBeDisabled()
})

test('test failures expose safe error details and config edits clear old results', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await page.getByLabel('API key *', { exact: true }).fill('fail')
  await page.getByRole('button', { name: 'Test connection', exact: true }).click()
  const failure = page.getByRole('button', { name: 'Connection failed', exact: true })
  await expect(failure).toBeVisible()
  await failure.hover()
  await expect(page.getByRole('tooltip')).toContainText('Authentication failed (401). Check the API key and try again.')
  await page.getByLabel('Base URL', { exact: true }).fill('https://api.example.com/v1')
  await expect(failure).not.toBeVisible()
})

test('creation notifies the consumer without requiring a connection test', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await page
    .getByRole('button', { name: 'Add provider', exact: true })
    .filter({ hasNot: page.locator('svg') })
    .click()
  await expect(page.getByRole('status', { name: 'Provider controller intent' })).toHaveText(
    'Create provider: OpenAI production · 2 models',
  )
  await expect(page.getByRole('dialog')).toHaveCount(0)
})

test('changing connection fields ignores an obsolete in-flight test result', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await page.getByRole('button', { name: 'Test connection', exact: true }).click()
  await page.getByLabel('API key *', { exact: true }).fill('replacement')
  await page.waitForTimeout(500)
  await expect(page.getByText('Check passed', { exact: true })).not.toBeVisible()
})

test('model drafts reject duplicate identifiers, invalid context and over-precision prices', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await page.getByRole('button', { name: 'Add custom model', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Add model', exact: true })
  await dialog.getByLabel('Model name', { exact: true }).fill('gpt-4.1')
  for (const label of ['1M Input', '1M Output', '1M Cached']) await dialog.getByLabel(label, { exact: true }).fill('0')
  await expect(dialog.getByRole('alert')).toHaveText('A model with this name already exists.')
  const save = dialog.getByRole('button', { name: 'Add model', exact: true })
  await expect(save).toBeDisabled()
  await dialog.getByLabel('Model name', { exact: true }).fill('new-model')
  await dialog.getByLabel('Max content window', { exact: true }).fill('-1')
  await expect(save).toBeDisabled()
  await dialog.getByLabel('Max content window', { exact: true }).fill('128000')
  await dialog.getByLabel('1M Input', { exact: true }).fill('0.123456789')
  await expect(save).toBeDisabled()
  await dialog.getByLabel('1M Input', { exact: true }).fill('0.12345678')
  await expect(save).toBeEnabled()
})

test('model dialog ignores outside clicks and protects changes on cancel and Escape', async ({ page }) => {
  await page.goto(`${story}configure-preset&viewMode=story`)
  await page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Edit model', exact: true })
  await dialog.getByLabel('1M Input', { exact: true }).fill('9.00')
  await page.mouse.click(5, 5)
  await expect(dialog).toBeVisible()
  await expect(page.getByRole('alertdialog')).toHaveCount(0)
  await dialog.getByRole('button', { name: 'Cancel', exact: true }).click()
  const warning = page.getByRole('alertdialog', { name: 'Discard changes?', exact: true })
  await expect(warning).toBeVisible()
  await warning.getByRole('button', { name: 'Keep editing', exact: true }).click()
  await expect(warning).not.toBeVisible()
  await expect(dialog.getByLabel('1M Input', { exact: true })).toHaveValue('9.00')
  await page.keyboard.press('Escape')
  await expect(warning).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(warning).not.toBeVisible()
  await expect(dialog.getByLabel('1M Input', { exact: true })).toHaveValue('9.00')
  await dialog.getByRole('button', { name: 'Close model popup', exact: true }).click()
  await warning.getByRole('button', { name: 'Discard changes', exact: true }).click()
  await expect(dialog).not.toBeVisible()
  await page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }).click()
  await expect(dialog.getByLabel('1M Input', { exact: true })).toHaveValue('2.00')
})

test('saved providers edit directly and save only actual changes', async ({ page }) => {
  await page.goto(`${story}saved-provider&viewMode=story`)
  const save = page.getByRole('button', { name: 'Save changes', exact: true })
  await expect(save).toBeDisabled()
  await expect(page.getByRole('button', { name: 'Edit', exact: true })).toHaveCount(0)
  await expect(page.getByLabel('API key', { exact: true })).toHaveValue('')
  await page.getByRole('searchbox', { name: 'Search models…' }).fill('gpt')
  await expect(save).toBeDisabled()
  await page.getByLabel('Connection name', { exact: true }).fill('OpenAI production updated')
  await expect(save).toBeEnabled()
  await page.getByLabel('Connection name', { exact: true }).fill('OpenAI production')
  await expect(save).toBeDisabled()
  await page.getByLabel('Connection name', { exact: true }).fill('OpenAI production updated')
  await save.click()
  await expect(save).toBeDisabled()
  await expect(page.getByRole('status', { name: 'Provider controller intent' })).toHaveText(
    'Save provider: OpenAI production updated',
  )
})

test('saved provider tracks model changes and cancel restores the saved configuration', async ({ page }) => {
  await page.goto(`${story}saved-provider&viewMode=story`)
  const save = page.getByRole('button', { name: 'Save changes', exact: true })
  await page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }).click()
  const dialog = page.getByRole('dialog', { name: 'Edit model', exact: true })
  await dialog.getByLabel('1M Input', { exact: true }).fill('3.00')
  await dialog.getByRole('button', { name: 'Save model', exact: true }).click()
  await expect(save).toBeEnabled()
  await page.getByRole('button', { name: 'Cancel', exact: true }).click()
  await expect(save).toBeDisabled()
  await expect(
    page.getByRole('row').filter({ has: page.getByRole('button', { name: 'Edit gpt-4.1', exact: true }) }),
  ).toContainText('$2.00')
  await page.getByLabel('API key', { exact: true }).fill('replacement')
  await expect(save).toBeEnabled()
  await page.getByLabel('API key', { exact: true }).fill('')
  await expect(save).toBeDisabled()
})

test('header clone action creates an independent draft without credentials or prior test results', async ({ page }) => {
  await page.goto(`${story}saved-provider&viewMode=story`)
  await page.getByRole('button', { name: 'Test connection', exact: true }).click()
  await expect(page.getByText('Check passed', { exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Provider actions', exact: true }).click()
  await page.getByRole('menuitem', { name: 'Clone provider', exact: true }).click()
  await expect(page.getByLabel('Connection name', { exact: true })).toHaveValue('OpenAI production clone')
  await expect(page.getByLabel('API key *', { exact: true })).toHaveValue('')
  await expect(page.getByText('Check passed', { exact: true })).not.toBeVisible()
  await expect(page.getByRole('checkbox', { name: 'Enable gpt-4.1', exact: true })).toBeChecked()
  const create = page.getByRole('button', { name: 'Create provider', exact: true })
  await expect(create).toBeDisabled()
  await page.getByLabel('API key *', { exact: true }).fill('new-key')
  await expect(create).toBeEnabled()
  await page.getByLabel('Connection name', { exact: true }).fill('Independent clone')
  await page.getByRole('button', { name: 'Cancel', exact: true }).click()
  await expect(page.getByLabel('Connection name', { exact: true })).toHaveValue('OpenAI production')
  await expect(page.getByRole('button', { name: 'Save changes', exact: true })).toBeDisabled()
})

test('Choose navigation opens configuration immediately with a 16px SVG', async ({ page }) => {
  await page.goto(`${story}choose-provider&viewMode=story`)
  await expect(page.getByRole('columnheader', { name: 'Choose', exact: true })).toBeVisible()
  const choose = page.getByRole('button', { name: 'Choose OpenAI', exact: true })
  await expect(choose.locator('svg')).toHaveCSS('width', '16px')
  await expect(choose.locator('svg')).toHaveCSS('height', '16px')
  await choose.click()
  await expect(page.getByLabel('Connection name', { exact: true })).toHaveValue('OpenAI production')
})
