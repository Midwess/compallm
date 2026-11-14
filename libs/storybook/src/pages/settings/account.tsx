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
export function AccessSettings() {
  const c = useSettingsController(),
    apps = useSettings((s) => s.applications)
  const others = useSettings((s) => s.hasOtherSessions)
  return (
    <SettingsContent>
      <Heading description="Manage your sessions and authorized applications.">Access</Heading>
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
                <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('revoke-sessions')}>
                  Revoke all
                </Button>
              ) : null}
            </Row>
          </Rows>
        </Section>
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
  const c = useSettingsController(),
    passkeys = useSettings((s) => s.passkeys)
  return (
    <SettingsContent>
      <Heading description="Manage your sign-in methods and API access.">Security</Heading>
      <div className="grid gap-12">
        <Section
          title="Passkeys"
          description="Sign in with your fingerprint, face, or security key."
          action={
            passkeys.length ? (
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('add-passkey')}>
                Add passkey
              </Button>
            ) : null
          }
        >
          {passkeys.length ? (
            <Rows>
              {passkeys.map((key, i) => (
                <Row key={key.id} label={key.name} description="Use this passkey to sign in." last={i === passkeys.length - 1}>
                  <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('remove-passkey', key.id)}>
                    Remove
                  </Button>
                </Row>
              ))}
            </Rows>
          ) : (
            <Empty
              title="No passkeys yet"
              action={<Button onClick={() => c.pages.dialogs.openDialog('add-passkey')}>Add passkey</Button>}
            >
              Add a passkey for a simpler sign-in experience.
            </Empty>
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
