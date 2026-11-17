if (window.isSecureContext && 'serviceWorker' in navigator) {
  void navigator.serviceWorker
    .register('/request-compression-sw.js', { scope: '/', updateViaCache: 'none' })
    .catch(() => undefined)
}
