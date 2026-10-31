import { useState } from 'react'
import type { Meta, StoryObj } from '@storybook/react-vite'
import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
import { StarIcon } from '@phosphor-icons/react/dist/csr/Star'
import { BellIcon } from '@phosphor-icons/react/dist/csr/Bell'
import { CopyIcon } from '@phosphor-icons/react/dist/csr/Copy'
import * as Navigation from '../../src/components/navigation'
import { Preview, Row, Sample } from './Preview'

const meta = { title: 'Components/Navigation controls', parameters: { layout: 'fullscreen' } } satisfies Meta
export default meta
type Story = StoryObj<typeof meta>
function Tabs() {
  const [selected, setSelected] = useState('Overview')
  return (
    <Row>
      {['Overview', 'Activity'].map((label) => (
        <Navigation.NavigationTab key={label} selected={selected === label} onClick={() => setSelected(label)}>
          {label}
        </Navigation.NavigationTab>
      ))}
    </Row>
  )
}
export const BreadcrumbTrail: Story = {
  render: () => (
    <Preview>
      <Sample label="Breadcrumb trail">
        <Navigation.BreadcrumbTrail items={[{ label: 'Projects', href: '#projects' }, { label: 'Compallm' }]} />
      </Sample>
    </Preview>
  ),
}
export const BackBreadcrumb: Story = {
  render: () => (
    <Preview>
      <Sample label="Back navigation">
        <Navigation.BackBreadcrumb href="#documents">Documents</Navigation.BackBreadcrumb>
      </Sample>
    </Preview>
  ),
}
export const ViewPill: Story = {
  render: () => (
    <Preview>
      <Sample label="Current view">
        <Navigation.ViewPill>All projects</Navigation.ViewPill>
      </Sample>
    </Preview>
  ),
}
export const AgentDock: Story = {
  render: () => (
    <Preview>
      <Sample label="Agent action">
        <Navigation.AgentDock />
      </Sample>
    </Preview>
  ),
}
export const NavigationTab: Story = {
  render: () => (
    <Preview>
      <Sample label="View selection">
        <Tabs />
      </Sample>
    </Preview>
  ),
}
export const ToolbarAction: Story = {
  render: () => (
    <Preview>
      <Sample label="Toolbar controls">
        <Row>
          <Navigation.ToolbarAction label="Filter">
            <FunnelIcon size={14} aria-hidden="true" />
          </Navigation.ToolbarAction>
          <Navigation.ToolbarAction label="More">
            <DotsThreeIcon size={14} aria-hidden="true" />
          </Navigation.ToolbarAction>
          <Navigation.ToolbarAction label="Favorite">
            <StarIcon size={14} aria-hidden="true" />
          </Navigation.ToolbarAction>
          <Navigation.ToolbarAction label="Notifications">
            <BellIcon size={14} aria-hidden="true" />
          </Navigation.ToolbarAction>
          <Navigation.ToolbarAction label="Copy URL">
            <CopyIcon size={14} aria-hidden="true" />
          </Navigation.ToolbarAction>
        </Row>
      </Sample>
    </Preview>
  ),
}
export const DragHandle: Story = {
  render: () => (
    <Preview>
      <Sample label="Handle · caller owns reordering">
        <Row>
          <Navigation.DragHandle label="Reorder item" />
          <Navigation.DragHandle label="Reorder unavailable" disabled />
        </Row>
      </Sample>
    </Preview>
  ),
}
