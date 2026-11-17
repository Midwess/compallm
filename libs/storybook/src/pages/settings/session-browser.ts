import Bowser from 'bowser'

export function describeSessionBrowser(userAgent?: string | null): string {
  if (!userAgent) return 'Unknown browser'
  const { browser, os } = Bowser.parse(userAgent)
  return [browser.name || 'Unknown browser', os.name].filter(Boolean).join(' on ')
}
