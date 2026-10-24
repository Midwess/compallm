import { defineConfig } from 'orval'

export default defineConfig({
  api: {
    input: {
      target: './openapi.json',
    },
    output: {
      target: './src/contract/api.ts',
      client: 'fetch',
      clean: true,
      override: {
        fetch: {
          forceSuccessResponse: true,
          includeHttpResponseReturnType: false,
        },
      },
    },
  },
})
