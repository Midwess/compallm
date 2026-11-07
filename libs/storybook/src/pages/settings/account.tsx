import { Button } from '../../components/buttons'
import { TextField } from '../../components/fields'
import { Avatar } from '../../components/content'
import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'
import { useSettings, useSettingsController } from './controller'

export function ProfileSettings() {
  const c = useSettingsController(),
    profile = useSettings((s) => s.profile),
    busy = useSettings((s) => s.busy)
  return (
    <SettingsContent>
      <Heading>Profile</Heading>
      <div className="grid gap-12">
        <Section title="Your profile" description="Manage how you appear across your organizations.">
          <Rows>
            <Row label="Profile picture" description="Choose a photo for your account">
              <div className="flex items-center gap-3">
                <Avatar name={profile.name} src={profile.avatar ?? undefined} />
                <label className="cursor-pointer rounded-lg border border-input-border px-3 py-1.5 text-xs">
                  Upload
                  <input
                    className="sr-only"
                    type="file"
                    accept="image/*"
                    aria-label="Upload profile photo"
                    onChange={(e) => c.setAvatar(e.target.files?.[0] ?? null)}
                  />
                </label>
                <Button variant="ghost" onClick={() => c.setAvatar(null)}>
                  Remove
                </Button>
              </div>
            </Row>
            <Row
              label={<label htmlFor="settings-full-name">Full name</label>}
              description="Your name as it appears in Compallm"
            >
              <TextField
                id="settings-full-name"
                className="w-70 max-w-full"
                value={profile.name}
                onChange={(e) => c.updateProfile({ name: e.target.value })}
              />
            </Row>
            <Row label={<label htmlFor="settings-username">Username</label>} description="Your preferred display name">
              <TextField
                id="settings-username"
                className="w-70 max-w-full"
                value={profile.username}
                onChange={(e) => c.updateProfile({ username: e.target.value })}
              />
            </Row>
            <Row label={<label htmlFor="settings-email">Email</label>} description="The email address for this account" last>
              <TextField
                id="settings-email"
                type="email"
                className="w-70 max-w-full"
                value={profile.email}
                onChange={(e) => c.updateProfile({ email: e.target.value })}
              />
            </Row>
          </Rows>
          <div className="flex justify-between">
            <Button onClick={() => c.signOut()}>Sign out</Button>
            <Button variant="primary" loading={busy} onClick={() => c.saveProfile()}>
              Save changes
            </Button>
          </div>
        </Section>
        <Section title="Account access" description="Manage sign-in methods and connected applications.">
          <Rows>
            <Row label="Security & access" description="Sessions, passkeys and authorized applications.">
              <Button variant="ghost" onClick={() => c.navigate('security')}>
                Manage
              </Button>
            </Row>
            <Row label="API keys" description="Manage keys for your organization membership." last>
              <Button variant="ghost" onClick={() => c.navigate('api-keys')}>
                Manage
              </Button>
            </Row>
          </Rows>
        </Section>
      </div>
    </SettingsContent>
  )
}
export function DangerSettings() {
  const c = useSettingsController()
  return (
    <SettingsContent>
      <Heading>Danger</Heading>
      <Section title="Delete account" description="Permanently delete your personal account.">
        <Rows>
          <Row label="Delete account" description="Manage the deletion of your Compallm account." last>
            <Button variant="ghost" className="text-error" onClick={() => c.openDialog('delete-account')}>
              Delete account
            </Button>
          </Row>
        </Rows>
      </Section>
    </SettingsContent>
  )
}
export function SecuritySettings() {
  const c = useSettingsController(),
    passkeys = useSettings((s) => s.passkeys),
    apps = useSettings((s) => s.applications)
  const others = useSettings((s) => s.hasOtherSessions)
  return (
    <SettingsContent>
      <Heading description="Manage your sign-in methods and connected applications.">Security & access</Heading>
      <div className="grid gap-12">
        <Section title="Sessions" description="Review and revoke access to your account.">
          <Rows>
            <Row label="Current session" description="You are signed in to Compallm.">
              <span className="text-xs">Current</span>
            </Row>
            <Row
              label="Other sessions"
              description={others ? 'Sign out of your other active sessions.' : 'No other active sessions.'}
              last
            >
              {others ? (
                <Button variant="ghost" onClick={() => c.openDialog('revoke-sessions')}>
                  Revoke all
                </Button>
              ) : null}
            </Row>
          </Rows>
        </Section>
        <Section
          title="Passkeys"
          description="Sign in with your fingerprint, face, or security key."
          action={
            passkeys.length ? (
              <Button variant="ghost" onClick={() => c.openDialog('add-passkey')}>
                Add passkey
              </Button>
            ) : null
          }
        >
          {passkeys.length ? (
            <Rows>
              {passkeys.map((key, i) => (
                <Row key={key.id} label={key.name} description="Use this passkey to sign in." last={i === passkeys.length - 1}>
                  <Button variant="ghost" onClick={() => c.openDialog('remove-passkey', key.id)}>
                    Remove
                  </Button>
                </Row>
              ))}
            </Rows>
          ) : (
            <Empty title="No passkeys yet" action={<Button onClick={() => c.openDialog('add-passkey')}>Add passkey</Button>}>
              Add a passkey for a simpler sign-in experience.
            </Empty>
          )}
        </Section>
        <Section title="Authorized applications" description="Review applications that can access your account.">
          {apps.length ? (
            <Rows>
              {apps.map((app, i) => (
                <Row key={app.id} label={app.name} description={`Granted access: ${app.scope}`} last={i === apps.length - 1}>
                  <Button variant="ghost" onClick={() => c.openDialog('application', app.id)}>
                    Review
                  </Button>
                </Row>
              ))}
            </Rows>
          ) : (
            <Empty title="No authorized applications">Applications you authorize will appear here.</Empty>
          )}
        </Section>
        <div className="px-4">
          <Button variant="ghost" onClick={() => c.navigate('api-keys')}>
            Manage API keys
          </Button>
        </div>
      </div>
    </SettingsContent>
  )
}
export function ApiKeySettings() {
  const c = useSettingsController(),
    keys = useSettings((s) => s.apiKeys),
    organization = useSettings((s) => s.organization.name)
  return (
    <SettingsContent>
      <Heading description={`Manage API access for your membership in ${organization}.`}>API keys</Heading>
      <Section
        title="Personal API keys"
        description="Keys are scoped to your current organization membership."
        action={keys.length ? <Button onClick={() => c.openDialog('create-key')}>Create API key</Button> : null}
      >
        {keys.length ? (
          <Rows>
            {keys.map((key, i) => (
              <Row key={key.id} label={key.name} description={key.prefix} last={i === keys.length - 1}>
                <Button variant="ghost" onClick={() => c.openDialog('revoke-key', key.id)}>
                  Revoke
                </Button>
              </Row>
            ))}
          </Rows>
        ) : (
          <Empty title="No API keys yet" action={<Button onClick={() => c.openDialog('create-key')}>Create API key</Button>}>
            Create a key to connect your tools to Compallm.
          </Empty>
        )}
      </Section>
      <p className="mt-4 px-4 text-[13px] leading-[22px] text-muted">
        Keep API keys private. Revoke a key when it is no longer needed.
      </p>
    </SettingsContent>
  )
}
