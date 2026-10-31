import type { Meta, StoryObj } from '@storybook/react-vite'
import { CalendarIcon } from '@phosphor-icons/react/dist/csr/Calendar'
import { FlagIcon } from '@phosphor-icons/react/dist/csr/Flag'
import * as Data from '../../src/components/data'
import { Preview, Row, Sample } from './Preview'

const meta = { title: 'Components/Data primitives', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
export const ProjectTableHeader: Story = {
  render: () => (
    <Preview>
      <Sample label="Column headers" wide>
        <Data.ProjectTable label="Project columns">
          <Data.ProjectTableHeader />
        </Data.ProjectTable>
      </Sample>
    </Preview>
  ),
}
export const ProjectTableRow: Story = {
  render: () => (
    <Preview>
      <Sample label="Project row" wide>
        <Data.ProjectTable label="Projects">
          <Data.ProjectTableHeader />
          <tbody>
            <Data.ProjectTableRow name="Compallm" lead="Alex Morgan" progress={35} />
            <Data.ProjectTableRow name="Example project" />
          </tbody>
        </Data.ProjectTable>
      </Sample>
    </Preview>
  ),
}
export const ProjectIcon: Story = {
  render: () => (
    <Preview>
      <Sample label="Project · 36 px">
        <Data.ProjectIcon />
      </Sample>
    </Preview>
  ),
}
export const ProjectHealth: Story = {
  render: () => (
    <Preview>
      <Sample label="Health status">
        <Row>
          <Data.ProjectHealth>No updates</Data.ProjectHealth>
          <Data.ProjectHealth tone="success">On track</Data.ProjectHealth>
          <Data.ProjectHealth tone="warning">At risk</Data.ProjectHealth>
        </Row>
      </Sample>
    </Preview>
  ),
}
export const ProjectProgress: Story = {
  render: () => (
    <Preview>
      <Sample label="Progress">
        <Row>
          <Data.ProjectProgress value={0} />
          <Data.ProjectProgress value={35} />
          <Data.ProjectProgress value={100} />
        </Row>
      </Sample>
    </Preview>
  ),
}
export const ResourceAction: Story = {
  render: () => (
    <Preview>
      <Sample label="Add resource">
        <Data.ResourceAction />
      </Sample>
    </Preview>
  ),
}
export const MilestoneAddAction: Story = {
  render: () => (
    <Preview>
      <Sample label="Add milestone">
        <Data.MilestoneAddAction />
      </Sample>
    </Preview>
  ),
}
export const ActivityEvent: Story = {
  render: () => (
    <Preview>
      <Sample label="Activity">
        <Data.ActivityEvent actor="Alex Morgan" dateTime="2026-10-27" timeLabel="Oct 27">
          created the project
        </Data.ActivityEvent>
      </Sample>
    </Preview>
  ),
}
export const PropertyChip: Story = {
  render: () => (
    <Preview>
      <Sample label="Property actions">
        <Row>
          <Data.PropertyChip icon={<CalendarIcon size={14} aria-hidden="true" />}>Target date</Data.PropertyChip>
          <Data.PropertyChip icon={<FlagIcon size={14} aria-hidden="true" />}>No priority</Data.PropertyChip>
        </Row>
      </Sample>
    </Preview>
  ),
}
export const PropertyRow: Story = {
  render: () => (
    <Preview>
      <Sample label="Property rows">
        <Data.PropertyRow label="Priority">
          <Data.PropertyChip>No priority</Data.PropertyChip>
        </Data.PropertyRow>
        <Data.PropertyRow label="Lead">
          <Data.PropertyChip>Add lead</Data.PropertyChip>
        </Data.PropertyRow>
      </Sample>
    </Preview>
  ),
}
