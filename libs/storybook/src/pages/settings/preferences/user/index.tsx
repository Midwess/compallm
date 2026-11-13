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
  const status = useUserPreference((state) => state.status)
  useEffect(() => configureUserPreferenceForm(controller, settings.apiError), [controller, settings])
  if (status !== 'ready')
    return (
      <SettingsContent>
        <Empty
          title={
            status === 'loading' || status === 'idle'
              ? 'Loading preferences…'
              : status === 'denied'
                ? 'Preferences are unavailable'
                : 'Could not load preferences'
          }
        >
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
  return <Form model={controller.store.form}>{children}</Form>
}
