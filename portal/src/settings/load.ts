export type SettingsRouteData = Awaited<ReturnType<typeof import('../lib/server/loads').settingsLoad>>['settingsRead']
