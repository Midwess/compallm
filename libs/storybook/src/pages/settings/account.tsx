import { FormError } from '../../components/form'
import { motion } from 'motion/react'
import { useTransition } from '../../components/motion'
import { Button } from '../../components/buttons'
import { TextField, FileField } from '../../components/fields'
import { Avatar } from '../../components/content'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { SettingsContent, Heading, Section, Rows, Row, Empty } from './shared'
import { useSettings, useSettingsController } from './controller'

export function ProfileSettings() {
  const timing = useTransition('interaction')
  const c = useSettingsController(),
    profile = useSettings((s) => s.profile, ['profile']),
    busy = useSettings((s) => s.busy)
  return (
    <SettingsContent>
      <Heading>Profile</Heading>
      <div className="grid gap-12">
        <Rows>
          <Row label="Profile picture" description="Choose a photo for your account">
            <motion.label
              initial="rest"
              whileHover="active"
              whileFocus="active"
              className="group relative inline-flex size-10 cursor-pointer rounded-full
focus-within:outline-2 focus-within:outline-ring"
            >
              <Avatar name={profile.name} src={profile.avatar ?? undefined} />
              <motion.span
                variants={{ rest: { opacity: 0 }, active: { opacity: 1 } }}
                transition={timing}
                aria-hidden="true"
                className="pointer-events-none absolute inset-0 grid place-items-center rounded-full bg-overlay/50
text-on-overlay
[@media(hover:none)]:opacity-100!"
              >
                <PlusIcon className="size-5" weight="bold" />
              </motion.span>
              <FileField
                name="profileUpload"
                className="sr-only"
                accept="image/jpeg,image/png,image/webp"
                aria-label="Upload profile photo"
                disabled={busy}
              />
              <FormError name="profileUpload" />
            </motion.label>
          </Row>
          <Row label={<label htmlFor="settings-full-name">Full name</label>} description="Your name as it appears in Compallm">
            <TextField id="settings-full-name" className="w-70 max-w-full" name="profile.name" />
            <FormError name="profile.name" />
          </Row>
          <Row label={<label htmlFor="settings-username">Username</label>} description="Your preferred display name">
            <TextField id="settings-username" className="w-70 max-w-full" name="profile.username" />
            <FormError name="profile.username" />
          </Row>
          <Row label={<label htmlFor="settings-email">Email</label>} description="The email address for this account" last>
            <TextField id="settings-email" type="email" className="w-70 max-w-full" value={profile.email} readOnly />
          </Row>
        </Rows>
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
            <Button variant="ghost" className="text-destructive" onClick={() => c.pages.dialogs.openDialog('delete-account')}>
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
                <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('revoke-sessions')}>
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
