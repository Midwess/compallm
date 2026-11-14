import { FormError } from '../../components/form'
import { useSyncExternalStore } from 'react'
import { Button } from '../../components/buttons'
import { TextField, UrlPreview } from '../../components/fields'
import { Select } from '../../components/dropdowns'
import { SettingsContent, Heading, Section, Rows, Row, Empty, AvatarUploadControl } from './shared'
import { useSettings, useSettingsController } from './controller'

const subscribeOrigin = () => () => {}
const browserOrigin = () => (typeof window === 'undefined' ? '' : window.location.origin)
const serverOrigin = () => ''

export function OrganizationSettings() {
  const origin = useSyncExternalStore(subscribeOrigin, browserOrigin, serverOrigin)
  const c = useSettingsController(),
    org = useSettings((s) => s.organization, ['organization']),
    savedName = useSettings((s) => s.organizationSavedName),
    status = useSettings((s) => s.organizationProfileStatus),
    canUpdate = useSettings((s) => s.organizationCanUpdateProfile),
    busy = useSettings((s) => s.organizationProfileSaving)
  const path = savedName || org.name ? `/${encodeURIComponent(savedName || org.name)}` : ''
  const organizationUrl = path && origin ? new URL(path, origin).href : path
  if (status !== 'ready')
    return (
      <SettingsContent>
        <Heading>Organization</Heading>
        <Empty
          title={
            status === 'loading'
              ? 'Loading organization…'
              : status === 'missing'
                ? 'Organization not found'
                : status === 'denied'
                  ? 'Organization access denied'
                  : 'Couldn’t load organization'
          }
          action={
            status === 'error' && c.pages.organization.reload ? (
              <Button onClick={c.pages.organization.reload}>Retry</Button>
            ) : undefined
          }
        >
          Organization profile is unavailable.
        </Empty>
      </SettingsContent>
    )
  return (
    <SettingsContent>
      <Heading>Organization</Heading>
      {!canUpdate ? (
        <p className="mb-4 px-4 text-xsm text-muted-foreground">You do not have permission to update this organization.</p>
      ) : null}
      <div className="grid gap-12">
        <Section title="Organization details" description="Your shared workspace identity.">
          <Rows>
            <Row label="Logo" description="Recommended size is 256×256px">
              <AvatarUploadControl
                name={org.name}
                saved={org.logo}
                field="organizationUpload"
                label="Upload organization logo"
                disabled={!canUpdate || busy}
                disabledReason={
                  !canUpdate ? 'You do not have permission to update this organization.' : 'Saving your organization.'
                }
              />
            </Row>
            <Row label={<label htmlFor="organization-name">Name</label>}>
              <TextField id="organization-name" className="w-70 max-w-full" name="organization.name" readOnly={!canUpdate} />
              <FormError name="organization.name" />
            </Row>
            <Row label="URL" last>
              <UrlPreview className="w-70 max-w-full" url={organizationUrl} onCopy={c.pages.dialogs.copy} />
            </Row>
          </Rows>
        </Section>
        <Section title="Danger zone" description="Manage deletion of this organization.">
          <Rows>
            <Row label="Delete organization" description="Manage deletion of this organization." last>
              <Button
                variant="ghost"
                className="text-destructive"
                onClick={() => c.pages.dialogs.openDialog('delete-organization')}
              >
                Delete organization
              </Button>
            </Row>
          </Rows>
        </Section>
        <div className="px-4">
          <Button variant="ghost" onClick={() => c.navigate('audit')}>
            Audit log →
          </Button>
        </div>
      </div>
    </SettingsContent>
  )
}

export function OrganizationOnboarding() {
  const org = useSettings((s) => s.organization, ['organization'])
  return (
    <SettingsContent>
      <Heading description="Onboarding updates are not available yet.">Onboarding</Heading>
      <Section title="Member onboarding" description="Configure the experience for your members.">
        <Rows>
          <Row label="Welcome message">
            <Button variant="ghost" disabled title="Onboarding updates are not available yet.">
              Edit
            </Button>
          </Row>
          <Row label="Announcements">
            <Button variant="ghost" disabled title="Onboarding updates are not available yet.">
              Edit
            </Button>
          </Row>
          <Row label="Default home view" last>
            <Select
              aria-label="Organization default home view"
              className="w-44"
              value={org.home}
              disabled
              options={['Chat', 'Models', 'Providers'].map((label) => ({ label, value: label }))}
            />
          </Row>
        </Rows>
      </Section>
    </SettingsContent>
  )
}
