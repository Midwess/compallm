export type ProviderRouteData = Awaited<ReturnType<typeof import('../lib/server/loads').providerLoad>>['providerRead']
