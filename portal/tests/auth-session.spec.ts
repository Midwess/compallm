import { expect, test } from '@playwright/test'

const user = { userId: 42, organizationId: 84, organizationName: 'Research team', sessionId: 7, organizationRole: 'owner' }

test('root checks the API and automatically redirects an existing session to its organization', async ({ page }) => {
  let checks = 0
  await page.route('**/api/auth/me', (route) => {
    checks += 1
    return route.fulfill({ json: user })
  })
  await page.goto('/')
  await expect(page).toHaveURL('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Create your account.', exact: true })).toHaveCount(0)
  expect(checks).toBe(1)
})

test('returning to root rechecks a previously signed-out session', async ({ page }) => {
  let signedIn = false
  await page.route('**/api/auth/me', (route) =>
    signedIn ? route.fulfill({ json: user }) : route.fulfill({ status: 401, json: { message: 'Authentication required' } }),
  )
  await page.goto('/login')
  await expect(page.getByRole('heading', { name: 'Welcome back.', exact: true })).toBeVisible()
  signedIn = true
  await page.getByRole('link', { name: 'Sign up', exact: true }).click()
  await expect(page).toHaveURL('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
})

test('signin checks current user before soft navigation and keeps identity in root memory', async ({ page }) => {
  let confirmed = false
  let meCalls = 0
  let confirmations = 0
  let release: () => void = () => {}
  const gate = new Promise<void>((resolve) => {
    release = resolve
  })
  await page.route('**/api/auth/me', async (route) => {
    meCalls += 1
    if (!confirmed) return route.fulfill({ status: 401, json: { message: 'Authentication required' } })
    await gate
    await route.fulfill({ json: user })
  })
  await page.route('**/api/auth/signin/email', (route) => route.fulfill({ json: {} }))
  await page.route('**/api/auth/signin/email/confirmation/otp', (route) => {
    confirmed = true
    confirmations += 1
    return route.fulfill({ json: {} })
  })
  let documents = 0
  page.on('request', (request) => {
    if (request.isNavigationRequest()) documents += 1
  })
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await page.getByRole('group', { name: 'Email code' }).getByRole('textbox').first().fill('123456')
  await expect.poll(() => meCalls).toBe(2)
  expect(page.url()).toContain('/login')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toHaveCount(0)
  release()
  await expect(page).toHaveURL('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  expect(confirmations).toBe(1)
  expect(documents).toBe(1)
  const stored = await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    return authController.store.getState()
  })
  expect(stored).toMatchObject({ currentUser: user, status: 'authenticated' })
  expect(await page.evaluate(() => localStorage.length + sessionStorage.length)).toBe(0)
})

test('reload restores session through one shared current-user request', async ({ page }) => {
  let calls = 0
  await page.route('**/api/auth/me', (route) => {
    calls += 1
    return route.fulfill({ json: user })
  })
  await page.goto('/Research%20team')
  await expect(page).toHaveURL('/Research%20team/chat')
  await expect(page.getByRole('button', { name: 'Switch workspace: Research team', exact: true })).toBeVisible()
  expect(calls).toBe(1)
  await page.reload()
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  expect(calls).toBe(2)
})

test('signed-out chat visits go to login without showing protected content', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.goto('/Research%20team/chat')
  await expect(page).toHaveURL('/login')
  await expect(page.getByRole('heading', { name: 'Welcome back.', exact: true })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toHaveCount(0)
})

test('server failure during session restoration does not render protected content or pretend signed out', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 503, json: { message: 'Session check unavailable' } }))
  await page.goto('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'Unable to load this page', exact: true })).toBeVisible()
  expect(page.url()).toContain('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toHaveCount(0)
})

test('identity-check retry does not repeat successful signup confirmation', async ({ page }) => {
  let meCalls = 0
  let confirmations = 0
  await page.route('**/api/auth/me', (route) => {
    meCalls += 1
    return meCalls === 1
      ? route.fulfill({ status: 401, json: { message: 'Authentication required' } })
      : meCalls === 2
        ? route.fulfill({ status: 503, json: { message: 'Try checking your session again' } })
        : route.fulfill({ json: user })
  })
  await page.route('**/api/auth/signup/email', (route) => route.fulfill({ json: {} }))
  await page.route('**/api/auth/signup/email/confirmation/otp', (route) => {
    confirmations += 1
    return route.fulfill({ json: {} })
  })
  await page.goto('/')
  await page.getByRole('textbox', { name: 'Email address' }).fill('new@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  const code = page.getByRole('group', { name: 'Email code' }).getByRole('textbox').first()
  await code.fill('123456')
  await expect(page.getByRole('alert')).toContainText('Try checking your session again')
  expect(page.url()).not.toContain('/Research%20team/chat')
  await code.fill('654321')
  await expect(page).toHaveURL('/Research%20team/chat')
  expect(confirmations).toBe(1)
})

test('workspace URL uses the verified organization name and corrects another name', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/another-team/chat')
  await expect(page).toHaveURL('/Research%20team/chat')
  await expect(page.getByRole('button', { name: 'Switch workspace: Research team', exact: true })).toBeVisible()
})

for (const response of ['unauthorized', 'null'] as const) {
  test(`root controller redirects an expired session on refresh: ${response}`, async ({ page }) => {
    let expired = false
    await page.route('**/api/auth/me', (route) => {
      if (!expired) return route.fulfill({ json: user })
      return response === 'unauthorized'
        ? route.fulfill({ status: 401, json: { message: 'Authentication required' } })
        : route.fulfill({ json: null })
    })
    await page.goto('/Research%20team/chat')
    await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
    expired = true
    await page.evaluate(async () => {
      const { authController } = await import('/src/auth/controller.ts')
      await authController.refreshCurrentUser()
    })
    await expect(page).toHaveURL('/login')
    await expect(page.getByRole('heading', { name: 'Welcome back.', exact: true })).toBeVisible()
    await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toHaveCount(0)
    const state = await page.evaluate(async () => {
      const { authController } = await import('/src/auth/controller.ts')
      return authController.store.getState()
    })
    expect(state).toMatchObject({ currentUser: null, status: 'anonymous', step: 'email', pending: null })
  })
}

test('root controller keeps server failures retryable without redirecting to signin', async ({ page }) => {
  let unavailable = false
  await page.route('**/api/auth/me', (route) =>
    unavailable
      ? route.fulfill({ status: 503, json: { message: 'Session check unavailable' } })
      : route.fulfill({ json: user }),
  )
  await page.goto('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  unavailable = true
  const status = await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    await authController.refreshCurrentUser().catch(() => {})
    return authController.store.getState().status
  })
  expect(status).toBe('error')
  await expect(page).toHaveURL('/Research%20team/chat')
  unavailable = false
  await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    await authController.refreshCurrentUser()
  })
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
})

test('authentication routes reuse the root controller and reset form state for their mode', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.route('**/api/auth/signin/email', (route) => route.fulfill({ json: {} }))
  let signupRequests = 0
  await page.route('**/api/auth/signup/email', (route) => {
    signupRequests += 1
    return route.fulfill({ json: {} })
  })
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('group', { name: 'Email code' })).toBeVisible()
  await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    authController.navigate?.('/')
  })
  await expect(page.getByRole('textbox', { name: 'Email address' })).toHaveValue('')
  await page.getByRole('textbox', { name: 'Email address' }).fill('new@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('group', { name: 'Email code' })).toBeVisible()
  expect(signupRequests).toBe(1)
})

test('identity refresh updates organization information without remounting workspace pages', async ({ page }) => {
  let name = user.organizationName
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: { ...user, organizationName: name } }))
  await page.goto('/Research%20team/chat')
  await page.getByRole('textbox', { name: 'Message', exact: true }).fill('Keep this draft')
  name = 'Renamed team'
  await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    await authController.refreshCurrentUser()
  })
  await expect(page.getByRole('button', { name: 'Switch workspace: Renamed team', exact: true })).toBeVisible()
  await expect(page.getByRole('textbox', { name: 'Message', exact: true })).toHaveValue('Keep this draft')
})

test('organization controller derives active information and isolates workspace resource state', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/chat')
  const result = await page.evaluate(async (identity) => {
    const { OrganizationController } = await import('/src/organization/controller.ts')
    const controller = new OrganizationController(identity, { navigate: () => {} }, { signOut: async () => {} })
    controller.store.setState({ providers: [{ id: 'provider', name: 'Provider', href: '#' }] })
    controller.setCurrentUser({ ...identity, organizationName: 'Renamed team' })
    const renamed = controller.getCurrentOrganization()
    const providersAfterRename = controller.store.getState().providers
    controller.setCurrentUser({ ...identity, organizationId: 85, organizationName: 'Another team' })
    return {
      renamed,
      providersAfterRename,
      switched: controller.getCurrentOrganization(),
      providersAfterSwitch: controller.store.getState().providers,
    }
  }, user)
  expect(result).toEqual({
    renamed: { id: '84', name: 'Renamed team' },
    providersAfterRename: [{ id: 'provider', name: 'Provider', href: '#' }],
    switched: { id: '85', name: 'Another team' },
    providersAfterSwitch: [],
  })
})

test('organization signout calls the root auth controller and clears identity after API success', async ({ page }) => {
  let signedOut = false
  let requests = 0
  await page.route('**/api/auth/me', (route) =>
    signedOut ? route.fulfill({ status: 401, json: { message: 'Authentication required' } }) : route.fulfill({ json: user }),
  )
  await page.route('**/api/auth/signout', (route) => {
    requests += 1
    signedOut = true
    expect(route.request().method()).toBe('POST')
    return route.fulfill({ status: 204 })
  })
  await page.goto('/Research%20team/chat')
  await page.getByRole('button', { name: 'Switch workspace: Research team', exact: true }).click()
  await page.getByRole('menuitem', { name: 'Sign out', exact: true }).click()
  await expect(page).toHaveURL('/login')
  await expect(page.getByRole('heading', { name: 'Welcome back.', exact: true })).toBeVisible()
  expect(requests).toBe(1)
  const state = await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    return authController.store.getState()
  })
  expect(state).toMatchObject({ currentUser: null, status: 'anonymous', email: '', code: '' })
})

test('failed signout preserves the session and can be retried', async ({ page }) => {
  let attempts = 0
  let signedOut = false
  await page.route('**/api/auth/me', (route) =>
    signedOut ? route.fulfill({ status: 401, json: { message: 'Authentication required' } }) : route.fulfill({ json: user }),
  )
  await page.route('**/api/auth/signout', (route) => {
    attempts += 1
    if (attempts === 1) return route.fulfill({ status: 503, json: { message: 'Please try signing out again' } })
    signedOut = true
    return route.fulfill({ status: 204 })
  })
  await page.goto('/Research%20team/chat')
  await page.getByRole('button', { name: 'Switch workspace: Research team', exact: true }).click()
  await page.getByRole('menuitem', { name: 'Sign out', exact: true }).click()
  await expect(page.getByRole('alert')).toContainText('Please try signing out again')
  await expect(page).toHaveURL('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Switch workspace: Research team', exact: true }).click()
  await page.getByRole('menuitem', { name: 'Sign out', exact: true }).click()
  await expect(page).toHaveURL('/login')
  expect(attempts).toBe(2)
})

test('concurrent controller calls share identity checks and signout requests', async ({ page }) => {
  let checks = 0
  let signouts = 0
  await page.route('**/api/auth/me', (route) => {
    checks += 1
    return route.fulfill({ json: user })
  })
  await page.route('**/api/auth/signout', (route) => {
    signouts += 1
    return route.fulfill({ status: 204 })
  })
  await page.goto('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  const initialChecks = checks
  const result = await page.evaluate(async () => {
    const { AuthController } = await import('/src/auth/controller.ts')
    const { urlController } = await import('/src/url/controller.ts')
    const controller = new AuthController({ store: urlController.store, navigate: () => {} })
    const identities = await Promise.all([
      controller.ensureCurrentUser(),
      controller.ensureCurrentUser(),
      controller.ensureCurrentUser(),
    ])
    await Promise.all([controller.signOut(), controller.signOut()])
    return { identities, state: controller.store.getState() }
  })
  expect(checks - initialChecks).toBe(1)
  expect(signouts).toBe(1)
  expect(result.identities).toEqual([user, user, user])
  expect(result.state).toMatchObject({ currentUser: null, status: 'anonymous' })
})

test('superseded identity checks cannot overwrite the latest controller state', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  const result = await page.evaluate(async () => {
    const { AuthController } = await import('/src/auth/controller.ts')
    const { urlController } = await import('/src/url/controller.ts')
    const controller = new AuthController({ store: urlController.store, navigate: () => {} })
    const stale = controller.refreshCurrentUser()
    const latest = controller.refreshCurrentUser()
    const outcomes = await Promise.allSettled([stale, latest])
    return {
      outcomes: outcomes.map((outcome) =>
        outcome.status === 'rejected'
          ? { status: outcome.status, name: outcome.reason.name }
          : { status: outcome.status, value: outcome.value },
      ),
      state: controller.store.getState(),
    }
  })
  expect(result.outcomes).toEqual([
    { status: 'rejected', name: 'AbortError' },
    { status: 'fulfilled', value: user },
  ])
  expect(result.state).toMatchObject({ currentUser: user, status: 'authenticated' })
})

test('API error messages preserve valid server text and reject malformed payloads', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  const messages = await page.evaluate(async () => {
    const { apiErrorMessage } = await import('/src/notifications/toast.tsx')
    return [
      undefined,
      null,
      'Raw exception',
      { message: 'Unwrapped error' },
      { info: null },
      { info: { message: 42 } },
      { info: { message: ' \n\t ' } },
      { info: { message: new String('Boxed message') } },
      { info: Object.assign(() => {}, { message: 'Function payload' }) },
      { info: { message: '  Keep the original server message.  ' } },
    ].map(apiErrorMessage)
  })
  expect(messages.slice(0, -1)).toEqual(Array(9).fill('Something went wrong. Please try again.'))
  expect(messages.at(-1)).toBe('  Keep the original server message.  ')
})

test('controller collection lookups select exact organization and model identifiers', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ json: user }))
  await page.goto('/Research%20team/chat')
  await expect(page.getByRole('heading', { name: 'New chat', exact: true })).toBeVisible()
  const result = await page.evaluate(async (identity) => {
    const { OrganizationController } = await import('/src/organization/controller.ts')
    const { ChatController } = await import('/src/chat/controller.ts')
    const workspace = new OrganizationController(identity, { navigate: () => {} }, { signOut: async () => {} })
    workspace.store.setState({
      organizations: [
        { id: 'other', name: 'Other team' },
        { id: '84', name: 'Selected team' },
      ],
    })
    const selected = workspace.getCurrentOrganization()
    workspace.store.setState({ organizationId: 'missing' })
    const missing = workspace.getCurrentOrganization()
    const chat = new ChatController()
    chat.store.setState({ models: [{ value: 'known-model', label: 'Known model' }], modelId: '' })
    chat.onModelChanged('unknown-model')
    const invalidModel = chat.store.getState().modelId
    chat.onModelChanged('known-model')
    return { selected, missing, invalidModel, validModel: chat.store.getState().modelId }
  }, user)
  expect(result).toEqual({
    selected: { id: '84', name: 'Selected team' },
    missing: null,
    invalidModel: '',
    validModel: 'known-model',
  })
})

test('OTP throttle drives resend countdown independently of expiry and a successful resend replaces the deadline', async ({
  page,
}) => {
  await page.clock.install()
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  let requests = 0
  await page.route('**/api/auth/signin/email', (route) => {
    requests += 1
    return route.fulfill({ json: { lifetime: { issuedAt: 100, expiresAt: 700, expiresInSeconds: 600, throttleSeconds: 3 } } })
  })
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Resend in 3s', exact: true })).toBeDisabled()
  await page.clock.runFor(1000)
  await expect(page.getByRole('button', { name: 'Resend in 2s', exact: true })).toBeDisabled()
  await page.clock.fastForward(10000)
  await page.getByRole('button', { name: 'Resend code', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Resend in 3s', exact: true })).toBeDisabled()
  expect(requests).toBe(2)
})

test('auth controller enforces the resend deadline and releases it when using another email', async ({ page }) => {
  await page.clock.install()
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  let requests = 0
  await page.route('**/api/auth/signup/email', (route) => {
    requests += 1
    return route.fulfill({ json: { lifetime: { issuedAt: 1, expiresAt: 601, expiresInSeconds: 600, throttleSeconds: 30 } } })
  })
  await page.goto('/')
  await page.getByRole('textbox', { name: 'Email address' }).fill('new@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Resend in 30s', exact: true })).toBeDisabled()
  await page.evaluate(async () => {
    const { authController } = await import('/src/auth/controller.ts')
    await authController.resendCode()
  })
  expect(requests).toBe(1)
  await page.getByRole('button', { name: 'Use a different email', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Continue', exact: true })).toBeEnabled()
})

test('zero token throttle allows immediate resend while expiry remains positive', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  let requests = 0
  await page.route('**/api/auth/signin/email', (route) => {
    requests += 1
    return route.fulfill({ json: { lifetime: { issuedAt: 1, expiresAt: 601, expiresInSeconds: 600, throttleSeconds: 0 } } })
  })
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await page.getByRole('button', { name: 'Resend code', exact: true }).click()
  await expect.poll(() => requests).toBe(2)
  await expect(page.getByRole('button', { name: 'Resend code', exact: true })).toBeEnabled()
})

for (const [entry, confirmation, endpoint] of [
  ['/login', '/login/confirmation', 'signin'],
  ['/signup', '/signup/confirmation', 'signup'],
] as const) {
  test(`${confirmation} restores the OTP step and countdown after reload without a current-user request`, async ({ page }) => {
    await page.clock.install()
    let userChecks = 0
    await page.route('**/api/auth/me', (route) => {
      userChecks += 1
      return route.fulfill({ status: 401, json: { message: 'Authentication required' } })
    })
    await page.route(`**/api/auth/${endpoint}/email`, (route) =>
      route.fulfill({ json: { lifetime: { issuedAt: 1, expiresAt: 601, expiresInSeconds: 600, throttleSeconds: 30 } } }),
    )
    await page.goto(entry)
    await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
    await page.getByRole('button', { name: 'Continue', exact: true }).click()
    await expect(page).toHaveURL(new RegExp(confirmation + '\\?'))
    const address = new URL(page.url())
    expect(address.searchParams.get('email')).toBe('alex@example.com')
    expect(address.searchParams.has('code')).toBe(false)
    expect(address.searchParams.has('token')).toBe(false)
    await page.clock.runFor(5000)
    await page.reload()
    await expect(page.getByRole('group', { name: 'Email code' })).toBeVisible()
    await expect(page.getByText('alex@example.com', { exact: true })).toBeVisible()
    await expect(page.getByRole('button', { name: 'Resend in 25s', exact: true })).toBeDisabled()
    expect(userChecks).toBe(1)
    await page.getByRole('button', { name: 'Use a different email', exact: true }).click()
    await expect(page).toHaveURL(entry)
    await expect(page.getByRole('textbox', { name: 'Email address' })).toBeVisible()
  })
}

test('confirmation URLs without a recipient return to email entry', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.goto('/login/confirmation')
  await expect(page).toHaveURL('/login')
  await expect(page.getByRole('textbox', { name: 'Email address' })).toBeVisible()
})

test('Back and Forward restore authentication entry and confirmation from their paths', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.route('**/api/auth/signin/email', (route) =>
    route.fulfill({ json: { lifetime: { issuedAt: 1, expiresAt: 601, expiresInSeconds: 600, throttleSeconds: 30 } } }),
  )
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page.getByRole('group', { name: 'Email code' })).toBeVisible()
  await expect(page).toHaveURL(/login\/confirmation\?/)
  await page.goBack()
  await expect(page.getByRole('textbox', { name: 'Email address' })).toBeVisible()
  await page.goForward()
  await expect(page.getByRole('group', { name: 'Email code' })).toBeVisible()
})

test('signup API errors appear below Continue with the original linked message and no toast', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  const message = 'Email already exists, <a href="/login">login here</a>'
  await page.route('**/api/auth/signup/email', (route) => route.fulfill({ status: 409, json: { message } }))
  let documents = 0
  page.on('request', (request) => {
    if (request.isNavigationRequest()) documents += 1
  })
  await page.goto('/signup')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  const submit = page.getByRole('button', { name: 'Continue', exact: true })
  await submit.click()
  const error = page.getByRole('alert')
  await expect(error).toHaveText('Email already exists, login here')
  const buttonBox = await submit.boundingBox(),
    errorBox = await error.boundingBox()
  expect(errorBox!.y).toBeGreaterThanOrEqual(buttonBox!.y + buttonBox!.height)
  await expect(page.locator('[data-ui="toast"]')).toHaveCount(0)
  const link = error.getByRole('link', { name: 'login here', exact: true })
  await expect(link).toHaveAttribute('href', '/login')
  await link.click()
  await expect(page).toHaveURL('/login')
  expect(documents).toBe(1)
})

test('API rich messages preserve safe content while removing executable markup and unsafe URLs', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  const message =
    '<strong>Try again</strong> <img src="x" onerror="window.messageExecuted=true">' +
    '<script>window.messageExecuted=true</script><a href="javascript:window.messageExecuted=true">unsafe</a> ' +
    '<a href="/login" onclick="window.messageExecuted=true">Sign in</a>'
  await page.route('**/api/auth/signin/email', (route) => route.fulfill({ status: 400, json: { message } }))
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  const error = page.getByRole('alert')
  await expect(error.locator('strong')).toHaveText('Try again')
  await expect(error.locator('img,script,[onclick]')).toHaveCount(0)
  await expect(error.locator('a').filter({ hasText: 'unsafe' })).not.toHaveAttribute('href')
  await expect(error.getByRole('link', { name: 'Sign in', exact: true })).toHaveAttribute('href', '/login')
  expect(await page.evaluate(() => 'messageExecuted' in window)).toBe(false)
  await page.getByRole('textbox', { name: 'Email address' }).fill('other@example.com')
  await expect(error).toHaveCount(0)
})

test('OTP API failure stays inline and retries without showing a toast', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.route('**/api/auth/signin/email', (route) =>
    route.fulfill({ json: { lifetime: { issuedAt: 1, expiresAt: 601, expiresInSeconds: 600, throttleSeconds: 0 } } }),
  )
  let attempts = 0
  await page.route('**/api/auth/signin/email/confirmation/otp', (route) => {
    attempts += 1
    return route.fulfill({ status: 401, json: { message: 'That code could not be verified. Please retry.' } })
  })
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await page.getByRole('group', { name: 'Email code' }).getByRole('textbox').first().fill('123456')
  await expect(page.getByRole('alert')).toHaveText('That code could not be verified. Please retry.')
  await expect(page.locator('[data-ui="toast"]')).toHaveCount(0)
  await page.getByRole('button', { name: 'Try again', exact: true }).click()
  await expect.poll(() => attempts).toBe(2)
})

test('inline auth failures reserve space and leave title, Continue, and account links still', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.route('**/api/auth/signup/email', (route) =>
    route.fulfill({
      status: 409,
      json: { message: 'Email already exists. '.repeat(20) + '<a href="/login">Sign in instead.</a>' },
    }),
  )
  await page.goto('/signup')
  await page.getByRole('textbox', { name: 'Email address' }).fill('alex@example.com')
  const title = page.getByRole('heading', { name: 'Create your account.', exact: true })
  const button = page.getByRole('button', { name: 'Continue', exact: true })
  const accountLink = page.getByRole('link', { name: 'Sign in', exact: true })
  const before = await Promise.all([title.boundingBox(), button.boundingBox(), accountLink.boundingBox()])
  await button.click()
  await expect(page.getByRole('alert')).toContainText('Email already exists.')
  const after = await Promise.all([title.boundingBox(), button.boundingBox(), accountLink.boundingBox()])
  expect(after).toEqual(before)
})

test('unknown signin emails show the API signup link without entering OTP confirmation', async ({ page }) => {
  await page.route('**/api/auth/me', (route) => route.fulfill({ status: 401, json: { message: 'Authentication required' } }))
  await page.route('**/api/auth/signin/email', (route) =>
    route.fulfill({ status: 404, json: { message: 'Email is not registered. <a href="/signup">Sign up here.</a>' } }),
  )
  await page.goto('/login')
  await page.getByRole('textbox', { name: 'Email address' }).fill('new@example.com')
  await page.getByRole('button', { name: 'Continue', exact: true }).click()
  await expect(page).toHaveURL('/login')
  await expect(page.getByRole('group', { name: 'Email code' })).toHaveCount(0)
  await page.getByRole('alert').getByRole('link', { name: 'Sign up here.', exact: true }).click()
  await expect(page).toHaveURL('/signup')
})
