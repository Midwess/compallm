import { useContext, useMemo } from 'react'
import Bowser from 'bowser'
import { SessionBrowserIcon } from './session-browser-icon'
import { List } from '../../components/list'
import { useFormStore } from '../../components/form'
import type { AuthController } from '../auth/types'
import { AuthControllerContext, PasskeyControllerContext } from '../../context'
import { PasskeySettings } from '../passkey'
import { FormError } from '../../components/form'
import { Button } from '../../components/buttons'
import { TextField } from '../../components/fields'
import { SettingsContent, Heading, Section, Rows, Row, Empty, AvatarUploadControl } from './shared'
import { useSettings, useSettingsController } from './controller'

export function ProfileSettings() {
  const c = useSettingsController(),
    profile = useSettings((s) => s.profile, ['profile']),
    busy = useSettings((s) => s.profileSaving),
    status = useSettings((s) => s.profileStatus)
  if (status !== 'ready')
    return (
      <SettingsContent>
        <Heading>Profile</Heading>
        <Empty
          action={
            status === 'error' && c.pages.profile.reload ? <Button onClick={c.pages.profile.reload}>Retry</Button> : undefined
          }
          title={
            status === 'loading'
              ? 'Loading profile…'
              : status === 'denied'
                ? 'Profile access denied'
                : status === 'missing'
                  ? 'Profile not found'
                  : 'Couldn’t load profile'
          }
        >
          {status === 'loading' ? 'Loading your saved profile.' : 'Your profile could not be loaded.'}
        </Empty>
      </SettingsContent>
    )
  return (
    <SettingsContent>
      <Heading>Profile</Heading>
      <div className="grid gap-12">
        <Rows>
          <Row label="Profile picture" description="Choose a photo for your account">
            <AvatarUploadControl
              name={profile.name || profile.username}
              saved={profile.avatar}
              field="profileUpload"
              label="Upload profile photo"
              disabled={busy}
              disabledReason="Saving your profile."
            />
          </Row>
          <Row label={<label htmlFor="settings-full-name">Full name</label>} description="Your name as it appears in Compallm">
            <TextField id="settings-full-name" className="w-70 max-w-full" name="profile.name" />
            <FormError name="profile.name" />
          </Row>
          <Row label={<label htmlFor="settings-username">Username</label>} description="Your unique username">
            <TextField id="settings-username" className="w-70 max-w-full" name="profile.username" />
            <FormError name="profile.username" />
          </Row>
          <Row label={<label htmlFor="settings-email">Email</label>} description="Email changes are not available yet." last>
            <TextField
              id="settings-email"
              type="email"
              variant="constant"
              className="w-70 max-w-full"
              value={profile.email}
              placeholder="Not available"
            />
          </Row>
        </Rows>
        <Section title="Danger zone" description="Manage your personal account.">
          <Rows>
            <Row label="Delete account" description="Manage the deletion of your Compallm account." last>
              <Button variant="ghost" className="text-destructive" onClick={() => c.pages.dialogs.openDialog('delete-account')}>
                Delete account
              </Button>
            </Row>
          </Rows>
        </Section>
      </div>
    </SettingsContent>
  )
}
function SessionItem({
  browser,
  location,
  current = false,
  lastSeen,
  ipAddress,
  onRevoke,
}: {
  browser: string
  location: string | null
  current?: boolean
  lastSeen?: string
  ipAddress?: string | null
  onRevoke?: () => void
}) {
  return (
    <div className="flex min-h-16 items-center justify-between gap-3 p-4">
      <div className="flex min-w-0 items-center gap-4">
        <span className="grid size-8 shrink-0 place-items-center rounded-md bg-elevated text-muted-foreground">
          <SessionBrowserIcon browser={browser} />
        </span>
        <div className="grid min-w-0 gap-0.5">
          <span className="truncate text-xsm leading-tight font-medium">{browser}</span>
          <div className="flex min-w-0 flex-wrap items-center gap-x-1 text-xs leading-tight text-muted-foreground">
            {current ? (
              <>
                <span className="inline-flex shrink-0 items-center gap-1 text-success">
                  <span className="size-1.5 rounded-full bg-success" aria-hidden="true" />
                  Current session
                </span>
                <span aria-hidden="true">·</span>
              </>
            ) : null}
            <span className="truncate" title={location ? `Approximate location: ${location}` : undefined}>
              {location ?? ipAddress ?? 'Location unavailable'}
            </span>
            {!current && lastSeen ? (
              <>
                <span aria-hidden="true">·</span>
                <span>Last seen {lastSeen}</span>
              </>
            ) : null}
          </div>
        </div>
      </div>
      {onRevoke ? (
        <Button variant="ghost" className="px-3" onClick={onRevoke}>
          {current ? 'Log out' : 'Revoke'}
        </Button>
      ) : null}
    </div>
  )
}

function CurrentSession({ controller }: { controller: AuthController }) {
  const userAgent = useFormStore(controller.store, (state) => state.currentUser?.userAgent)
  const ipAddress = useFormStore(controller.store, (state) => state.currentUser?.ipAddress)
  const sessionId = useFormStore(controller.store, (state) => state.currentUser?.id)
  const saved = useSettings((state) => state.sessionLocation)
  const settings = useSettingsController()
  const location = saved && saved.sessionId === sessionId && saved.ipAddress === ipAddress ? saved.location : null
  const browser = useMemo(() => {
    if (!userAgent) return 'Unknown browser'
    const { browser, os } = Bowser.parse(userAgent)
    return [browser.name || 'Unknown browser', os.name].filter(Boolean).join(' on ')
  }, [userAgent])
  return <SessionItem browser={browser} location={location} ipAddress={ipAddress} current onRevoke={settings.signOut} />
}

function SessionSettings() {
  const controller = useContext(AuthControllerContext)
  const settings = useSettingsController()
  const otherSessions = useSettings((state) => state.otherSessions)
  const location = useSettings((state) => state.sessionLocation)
  return (
    <Section title="Sessions" description="Devices logged into your account">
      <div className="grid gap-3">
        <Rows className="shadow-control">
          {controller ? (
            <CurrentSession controller={controller} />
          ) : (
            <SessionItem browser="Unknown browser" location={null} current onRevoke={settings.signOut} />
          )}
        </Rows>
        {otherSessions.length ? (
          <Rows className="shadow-control">
            <div className="flex min-h-16 items-center justify-between gap-4 border-b border-border p-4">
              <span className="text-xsm font-medium">
                {otherSessions.length} other {otherSessions.length === 1 ? 'session' : 'sessions'}
              </span>
              <Button variant="ghost" className="px-3" onClick={() => settings.pages.dialogs.openDialog('revoke-sessions')}>
                Revoke all
              </Button>
            </div>
            <List.Root>
              <List.Items items={otherSessions} getKey={(session) => session.id} className="m-0 list-none p-0">
                {(session) => (
                  <div
                    className={
                      session.id === otherSessions[0].id
                        ? undefined
                        : 'relative before:absolute before:inset-x-4 before:top-0 before:border-t before:border-border'
                    }
                  >
                    <SessionItem
                      browser={session.browser}
                      location={session.location}
                      ipAddress={session.ipAddress}
                      lastSeen={session.lastSeen}
                      onRevoke={settings.revokeSession ? () => settings.revokeSession!(session.id) : undefined}
                    />
                  </div>
                )}
              </List.Items>
            </List.Root>
          </Rows>
        ) : null}
        {location || otherSessions.some((session) => session.location) ? (
          <a
            href="https://db-ip.com"
            target="_blank"
            rel="noreferrer"
            className="justify-self-end px-4 text-xs text-muted-foreground hover:text-foreground"
          >
            IP Geolocation by DB-IP
          </a>
        ) : null}
      </div>
    </Section>
  )
}

export function AccessSettings() {
  const c = useSettingsController(),
    apps = useSettings((s) => s.applications)
  return (
    <SettingsContent>
      <Heading description="Manage your sessions and authorized applications.">Access</Heading>
      <div className="grid gap-12">
        <SessionSettings />
        <Section title="Authorized applications" description="Review applications that can access your account.">
          {apps.length ? (
            <Rows>
              {apps.map((app, i) => (
                <Row key={app.id} label={app.name} description={`Granted access: ${app.scope}`} last={i === apps.length - 1}>
                  <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('application', app.id)}>
                    Review
                  </Button>
                </Row>
              ))}
            </Rows>
          ) : (
            <Empty title="No authorized applications">Applications you authorize will appear here.</Empty>
          )}
        </Section>
      </div>
    </SettingsContent>
  )
}
export function SecuritySettings() {
  const c = useSettingsController()
  const passkeys = useContext(PasskeyControllerContext)
  const previewPasskeys = useSettings((state) => state.passkeys)
  const applications = useSettings((state) => state.applications)
  const keys = useSettings((state) => state.apiKeys)
  return (
    <SettingsContent>
      <Heading>Security &amp; access</Heading>
      <div className="grid gap-12">
        <SessionSettings />
        {passkeys ? (
          <PasskeySettings />
        ) : (
          <Section
            title="Passkeys"
            description="Passkeys are a secure way to sign in to your Compallm account."
            action={
              previewPasskeys.length ? (
                <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('add-passkey')}>
                  New passkey
                </Button>
              ) : undefined
            }
          >
            {previewPasskeys.length ? (
              <Rows>
                {previewPasskeys.map((key, index) => (
                  <Row key={key.id} label={key.name} last={index === previewPasskeys.length - 1}>
                    <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('remove-passkey', key.id)}>
                      Revoke
                    </Button>
                  </Row>
                ))}
              </Rows>
            ) : (
              <Empty
                title="No passkeys registered"
                action={<Button onClick={() => c.pages.dialogs.openDialog('add-passkey')}>New passkey</Button>}
              >
                Add a passkey for a simpler sign-in experience.
              </Empty>
            )}
          </Section>
        )}
        <Section title="Personal API keys" description="Use API keys to connect your tools to Compallm.">
          <Rows>
            <Row
              label={keys.length ? `${keys.length} API keys` : 'No API keys created'}
              description="Review your personal API access."
              last
            >
              <Button variant="ghost" onClick={() => c.navigate('api-keys')}>
                Manage API keys
              </Button>
            </Row>
          </Rows>
        </Section>
        <Section title="Authorized applications" description="Applications you’ve approved">
          {applications.length ? (
            <Rows>
              {applications.map((app, index) => (
                <Row
                  key={app.id}
                  label={app.name}
                  description={`Granted access: ${app.scope}`}
                  last={index === applications.length - 1}
                >
                  <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('application', app.id)}>
                    Review
                  </Button>
                </Row>
              ))}
            </Rows>
          ) : (
            <Empty title="No authorized applications">Applications you authorize will appear here.</Empty>
          )}
        </Section>
      </div>
    </SettingsContent>
  )
}
export function ApiKeySettings() {
  const c = useSettingsController(),
    keys = useSettings((s) => s.apiKeys),
    organization = useSettings((s) => s.organization.name, ['organization.name'])
  return (
    <SettingsContent>
      <Heading description={`Manage API access for your membership in ${organization}.`}>API keys</Heading>
      <Section
        title="Personal API keys"
        description="Keys are scoped to your current organization membership."
        action={keys.length ? <Button onClick={() => c.pages.dialogs.openDialog('create-key')}>Create API key</Button> : null}
      >
        {keys.length ? (
          <Rows>
            {keys.map((key, i) => (
              <Row key={key.id} label={key.name} description={key.prefix} last={i === keys.length - 1}>
                <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('revoke-key', key.id)}>
                  Revoke
                </Button>
              </Row>
            ))}
          </Rows>
        ) : (
          <Empty
            title="No API keys yet"
            action={<Button onClick={() => c.pages.dialogs.openDialog('create-key')}>Create API key</Button>}
          >
            Create a key to connect your tools to Compallm.
          </Empty>
        )}
      </Section>
      <p className="mt-4 px-4 text-xsm leading-5.5 text-muted-foreground">
        Keep API keys private. Revoke a key when it is no longer needed.
      </p>
    </SettingsContent>
  )
}
