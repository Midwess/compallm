const publicEndpoint = 'https://compallm-geolocation-production.up.railway.app'

export async function lookupSessionLocation(
  ipAddress: string,
  signal: AbortSignal,
  endpoint = import.meta.env.VITE_GEOLOCATION_URL ?? publicEndpoint,
): Promise<string | null> {
  if (!endpoint || signal.aborted) return null
  try {
    const response = await fetch(`${endpoint.replace(/\/$/, '')}/v1/locations`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ ip: ipAddress }),
      credentials: 'omit',
      cache: 'no-store',
      redirect: 'error',
      signal: AbortSignal.any([signal, AbortSignal.timeout(1500)]),
    })
    if (!response.ok || Number(response.headers.get('content-length')) > 4096) return null
    const result: unknown = await response.json()
    if (!result || typeof result !== 'object' || !('location' in result)) return null
    return typeof result.location === 'string' && result.location.length <= 256 ? result.location : null
  } catch {
    return null
  }
}
