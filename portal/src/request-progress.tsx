import { useStore } from 'zustand'
import { PageProgress } from '@compallm/storybook/components/page-progress'
import { requestActivity } from './request-activity'

export function RequestProgress() {
  const active = useStore(requestActivity, (state) => state.pending > 0)
  return <PageProgress active={active} />
}
