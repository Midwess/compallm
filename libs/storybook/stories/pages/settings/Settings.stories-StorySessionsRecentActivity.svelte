<script lang="ts">
  import Demo from './Settings.stories-Demo.svelte'
  const now = Math.floor(Date.now() / 1000)
</script>

<Demo
  page="access"
  session={{
    ipAddress: '203.0.113.42',
    userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36',
    signedInAt: String(now - 7200),
    lastSeenAt: String(now - 60),
  }}
  initial={{
    otherSessions: [
      {
        id: 'recent',
        browser: 'Firefox on macOS',
        location: 'Ho Chi Minh City, VN',
        lastSeenAt: String(now - 3600),
        signedInAt: String(now - 86400 * 3),
      },
      {
        id: 'older',
        browser: 'Safari on iOS',
        location: null,
        ipAddress: '203.0.113.43',
        lastSeenAt: String(now - 86400),
        signedInAt: String(now - 86400 * 7),
      },
    ],
  }}
/>
