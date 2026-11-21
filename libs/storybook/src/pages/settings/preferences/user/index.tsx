import { LoadingRegion } from '../../../../components/loading'
import { useEffect, type ReactNode } from 'react'
import { Form } from '../../../../components/form'
import { Button } from '../../../../components/buttons'
import { SettingsContent, Empty } from '../../shared'
import { useSettingsController } from '../../controller'
import { useUserPreference, useUserPreferenceController } from './controller'
import { configureUserPreferenceForm } from './form'

export function UserPreferencePage({ children }: { children: ReactNode }) {
  const controller = useUserPreferenceController()
  const settings = useSettingsController()
  const saved = useUserPreference((state) => state.saved)
  const ready = !!saved
  const status = useUserPreference((state) => state.status)
  useEffect(
    () => (ready ? configureUserPreferenceForm(controller, settings.apiError) : undefined),
    [controller, settings, ready],
  )
  if (status === 'denied' || (status === 'error' && !saved))
    return (
      <SettingsContent>
        <Empty title={status === 'denied' ? 'Preferences are unavailable' : 'Could not load preferences'}>
          {status === 'denied'
            ? 'Your session cannot access these preferences.'
            : status === 'error'
              ? 'Try loading your saved preferences again.'
              : 'Loading your saved preferences.'}
        </Empty>
        {status === 'error' || status === 'denied' ? (
          <Button variant="ghost" onClick={() => void controller.load()}>
            Retry
          </Button>
        ) : null}
      </SettingsContent>
    )
  return (
    <LoadingRegion ready={!!saved} pending={status === 'loading' || status === 'idle'} label="Loading preferences">
      <Form.Root model={controller.store.form}>{children}</Form.Root>
    </LoadingRegion>
  )
}
