import { FormError } from '../../components/form'
import { Button } from '../../components/buttons'
import { TextField, FileField } from '../../components/fields'
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
    org = useSettings((s) => s.organization, ['organization'])
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
                  <FileField
                    name="organizationUpload"
                    accept="image/jpeg,image/png,image/webp"
                    className="sr-only"
                    aria-label="Upload organization logo"
                  />
                  <FormError name="organizationUpload" />
                </label>
              </div>
            </Row>
            <Row label={<label htmlFor="organization-name">Name</label>}>
              <TextField id="organization-name" className="w-70 max-w-full" name="organization.name" />
              <FormError name="organization.name" />
            </Row>
            <Row label={<label htmlFor="organization-url">URL</label>} last>
              <TextField id="organization-url" className="w-70 max-w-full" name="organization.slug" />
              <FormError name="organization.slug" />
            </Row>
          </Rows>
        </Section>
        <Section title="Time & region" description="Defaults used for organization reporting.">
          <Rows>
            <Row label="First month of the fiscal year" description="Used for fiscal reporting periods.">
              <Select
                aria-label="First month of the fiscal year"
                className="w-44"
                name="organization.fiscalMonth"
                options={months.map((label) => ({ label, value: label }))}
              />
              <FormError name="organization.fiscalMonth" />
            </Row>
            <Row label="Region" description="Deployment region for this organization." last>
              <span className="text-xs text-muted">{org.region}</span>
            </Row>
          </Rows>
        </Section>
        <Section title="Member onboarding" description="Configure the experience for your members.">
          <Rows>
            <Row label="Welcome message">
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('welcome')}>
                Edit
              </Button>
            </Row>
            <Row label="Announcements">
              <Button variant="ghost" onClick={() => c.pages.dialogs.openDialog('announcements')}>
                Edit
              </Button>
            </Row>
            <Row label="Default home view" last>
              <Select
                aria-label="Organization default home view"
                className="w-44"
                name="organization.home"
                options={['Chat', 'Models', 'Providers'].map((label) => ({ label, value: label }))}
              />
              <FormError name="organization.home" />
            </Row>
          </Rows>
        </Section>
        <Section title="Danger zone" description="Manage deletion of this organization.">
          <Rows>
            <Row label="Delete organization" description="Manage deletion of this organization." last>
              <Button variant="ghost" className="text-error" onClick={() => c.pages.dialogs.openDialog('delete-organization')}>
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
