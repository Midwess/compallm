import type { RequestActivity } from './graphql-client'

declare global {
  namespace App {
    interface Locals {
      appearance?: { theme: string; dark: boolean; size: string; underline: boolean }
      portal: import('./lib/server/portal').PortalRequest
    }
    interface Metadata {
      requestActivity?: RequestActivity
    }
  }
}
export {}
