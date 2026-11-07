import { Button } from '../../components/buttons'
import { TextField } from '../../components/fields'
import { Select } from '../../components/dropdowns'
import { Avatar } from '../../components/content'
import { SettingsContent, Heading, Section, Rows, Row } from './shared'
import { useSettings, useSettingsController } from './controller'

const months = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
]
export function OrganizationSettings() {
  const c = useSettingsController(),
    org = useSettings((s) => s.organization)
  return (
    <SettingsContent>
      <Heading>Organization</Heading>
      <div className="grid gap-12">
        <Section title="Organization details" description="Your shared workspace identity.">
          <Rows>
            <Row label="Logo" description="Recommended size is 256×256px">
              <div className="flex items-center gap-3">
                <Avatar name={org.name} src={org.logo ?? undefined} />
                <label className="cursor-pointer rounded-lg border border-input-border px-3 py-1.5 text-xs">
                  Upload
                  <input
                    type="file"
                    accept="image/*"
                    className="sr-only"
                    aria-label="Upload organization logo"
                    onChange={(e) => c.setAvatar(e.target.files?.[0] ?? null, true)}
                  />
                </label>
              </div>
            </Row>
            <Row label={<label htmlFor="organization-name">Name</label>}>
              <TextField
                id="organization-name"
                className="w-70 max-w-full"
                value={org.name}
                onChange={(e) => c.updateOrganization({ name: e.target.value })}
              />
            </Row>
            <Row label={<label htmlFor="organization-url">URL</label>} last>
              <TextField
                id="organization-url"
                className="w-70 max-w-full"
                value={org.slug}
                onChange={(e) => c.updateOrganization({ slug: e.target.value })}
              />
            </Row>
          </Rows>
        </Section>
        <Section title="Time & region" description="Defaults used for organization reporting.">
          <Rows>
            <Row label="First month of the fiscal year" description="Used for fiscal reporting periods.">
              <Select
                aria-label="First month of the fiscal year"
                className="w-44"
                value={org.fiscalMonth}
                options={months.map((label) => ({ label, value: label }))}
                onValueChange={(fiscalMonth) => c.updateOrganization({ fiscalMonth })}
              />
            </Row>
            <Row label="Region" description="Deployment region for this organization." last>
              <span className="text-xs text-muted">{org.region}</span>
            </Row>
          </Rows>
        </Section>
        <Section title="Member onboarding" description="Configure the experience for your members.">
          <Rows>
            <Row label="Welcome message">
              <Button variant="ghost" onClick={() => c.openDialog('welcome')}>
                Edit
              </Button>
            </Row>
            <Row label="Announcements">
              <Button variant="ghost" onClick={() => c.openDialog('announcements')}>
                Edit
              </Button>
            </Row>
            <Row label="Default home view" last>
              <Select
                aria-label="Organization default home view"
                className="w-44"
                value={org.home}
                options={['Chat', 'Models', 'Providers'].map((label) => ({ label, value: label }))}
                onValueChange={(home) => c.updateOrganization({ home })}
              />
            </Row>
          </Rows>
        </Section>
        <Section title="Danger zone" description="Manage deletion of this organization.">
          <Rows>
            <Row label="Delete organization" description="Manage deletion of this organization." last>
              <Button variant="ghost" className="text-error" onClick={() => c.openDialog('delete-organization')}>
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
