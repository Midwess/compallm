import { expect, test } from '@playwright/test'

test('the workspace starts with an empty story index and a working foundation theme', async ({ page, request }) => {
  const response = await request.get('/index.json')
  expect(response.ok()).toBe(true)
  const index = await response.json() as { entries: Record<string, unknown> }
  expect(index.entries).toEqual({})

  const errors: string[] = []
  const backendRequests: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  page.on('request', request => {
    if (new URL(request.url()).pathname.startsWith('/api/')) backendRequests.push(request.url())
  })
  await page.goto('/iframe.html')
  await expect(page.locator('#storybook-root')).toBeEmpty()
  await expect(page.locator('body')).toHaveCSS('background-color', 'rgb(9, 9, 10)')
  await expect(page.locator('body')).toHaveCSS('color', 'rgb(228, 229, 231)')

  // Exercise compiled utilities without registering a story or shipping a demo.
  await page.evaluate(() => {
    const probe = document.createElement('div')
    probe.id = 'foundation-probe'
    probe.className = 'flex gap-space-16 p-space-24 m-space-4 size-[64px] bg-primary text-on-primary'
    probe.style.visibility = 'hidden'
    document.body.append(probe)
  })
  const probe = page.locator('#foundation-probe')
  await expect(probe).toHaveCSS('gap', '8px')
  await expect(probe).toHaveCSS('padding', '12px')
  await expect(probe).toHaveCSS('margin', '2px')
  await expect(probe).toHaveCSS('width', '64px')
  await expect(probe).toHaveCSS('background-color', 'rgb(94, 106, 210)')
  await expect(probe).toHaveCSS('color', 'rgb(255, 255, 255)')
  await page.evaluate(() => document.documentElement.style.setProperty('--layout-spacing-scale', '1'))
  await expect(probe).toHaveCSS('gap', '16px')
  await expect(probe).toHaveCSS('padding', '24px')
  await expect(probe).toHaveCSS('width', '64px')
  await page.evaluate(() => {
    document.documentElement.style.removeProperty('--layout-spacing-scale')
    const probe = document.querySelector('#foundation-probe')!
    probe.className = 'p-0 m-0 gap-0 size-[32px]'
  })
  await expect(probe).toHaveCSS('padding', '0px')
  await expect(probe).toHaveCSS('margin', '0px')
  await expect(probe).toHaveCSS('gap', '0px')
  await expect(probe).toHaveCSS('width', '32px')
  await probe.evaluate(element => element.remove())
  expect(errors).toEqual([])
  expect(backendRequests).toEqual([])
})
