import { Agent, request } from 'node:http'
import { Readable } from 'node:stream'

const connections = new Agent({ keepAlive: true, maxSockets: 128, maxFreeSockets: 16 })

export function requestGraphql(url: URL, headers: Headers, body: string, signal: AbortSignal): Promise<Response> {
  return new Promise((resolve, reject) => {
    const outgoing = request(
      url,
      { method: 'POST', headers: Object.fromEntries(headers), agent: connections, signal },
      (incoming) => {
        const received = new Headers()
        for (let index = 0; index < incoming.rawHeaders.length; index += 2)
          received.append(incoming.rawHeaders[index], incoming.rawHeaders[index + 1])
        resolve(
          new Response(Readable.toWeb(incoming) as ReadableStream<Uint8Array>, {
            status: incoming.statusCode ?? 502,
            headers: received,
          }),
        )
      },
    )
    outgoing.on('error', reject)
    outgoing.end(body)
  })
}
