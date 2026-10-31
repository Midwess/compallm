import type { Meta, StoryObj } from '@storybook/react-vite'
import * as Content from '../../src/components/content'
import { Switch } from '../../src/components/selection'
import { Preview, Row, Sample } from './Preview'

const meta = { title: 'Components/Content & code', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const SectionHeading: Story = {
  render: () => (
    <Preview>
      <Sample label="Section heading" wide>
        <Content.SectionHeading description="A short description of this section.">Section title</Content.SectionHeading>
      </Sample>
    </Preview>
  ),
}
export const SettingsRow: Story = {
  render: () => (
    <Preview>
      <Sample label="Settings row · 65 px" wide>
        <Content.SettingsRow
          label={<label htmlFor="descriptions">Show descriptions</label>}
          description="Explain what this preference changes."
        >
          <Switch id="descriptions" defaultChecked />
        </Content.SettingsRow>
      </Sample>
    </Preview>
  ),
}
export const Avatar: Story = {
  render: () => (
    <Preview>
      <Sample label="Initials">
        <Row>
          <Content.Avatar name="Alex Morgan" />
          <Content.Avatar name="Sam Lee" size={24} />
        </Row>
      </Sample>
    </Preview>
  ),
}
export const Badge: Story = {
  render: () => (
    <Preview>
      <Sample label="Status tones">
        <Row>
          <Content.Badge>Member</Content.Badge>
          <Content.Badge tone="success">Active</Content.Badge>
          <Content.Badge tone="warning">Pending</Content.Badge>
          <Content.Badge tone="error">Failed</Content.Badge>
        </Row>
      </Sample>
    </Preview>
  ),
}
export const CodePreview: Story = {
  render: () => (
    <Preview>
      <Sample label="Code text" wide>
        <Content.CodePreview
          language="TypeScript"
          code={'async function loadItems() {\n  const response = await fetch("/items");\n  return response.json();\n}'}
        />
      </Sample>
    </Preview>
  ),
}
export const DiffLine: Story = {
  render: () => (
    <Preview>
      <Sample label="Context, added, removed" wide>
        <Content.DiffLine lineNumber={10}>const response = await loadItems();</Content.DiffLine>
        <Content.DiffLine lineNumber={11} change="added">
          return response.data;
        </Content.DiffLine>
        <Content.DiffLine lineNumber={12} change="removed">
          return response.json();
        </Content.DiffLine>
      </Sample>
    </Preview>
  ),
}
export const MemberRow: Story = {
  render: () => (
    <Preview>
      <Sample label="Member identity" wide>
        <Content.MemberRow name="Alex Morgan" email="alex@example.com" role="Member" />
      </Sample>
    </Preview>
  ),
}
export const EditTimestamp: Story = {
  render: () => (
    <Preview>
      <Sample label="Last edited">
        <Content.EditTimestamp dateTime="2026-10-31T09:00:00Z">just now</Content.EditTimestamp>
      </Sample>
    </Preview>
  ),
}
export const ChangedFile: Story = {
  render: () => (
    <Preview>
      <Sample label="Changed file">
        <Content.ChangedFile path="src/response.ts" added={2} removed={1} />
      </Sample>
    </Preview>
  ),
}
export const DocumentIcon: Story = {
  render: () => (
    <Preview>
      <Sample label="Document · 24 px">
        <Content.DocumentIcon />
      </Sample>
    </Preview>
  ),
}
