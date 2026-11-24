export const subscribeOrigin = () => () => {}
export const browserOrigin = () => (typeof window === 'undefined' ? '' : window.location.origin)
export const serverOrigin = () => ''
export { default as OrganizationSettings } from './organization-OrganizationSettings.svelte'
export { default as OrganizationOnboarding } from './organization-OrganizationOnboarding.svelte'
