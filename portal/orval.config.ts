import { defineConfig } from 'orval'

export default defineConfig({
  api: {
    input: {
      target: './openapi.json',
    },
    output: {
      target: '../libs/contract/api.ts',
      client: 'fetch',
      clean: false,
      override: {
        fetch: {
          forceSuccessResponse: true,
          includeHttpResponseReturnType: false,
        },
      },
    },
  },
  validation: {
    input: {
      target: './openapi.json',
      override: { transformer: './contract-transformer.cjs' },
    },
    output: {
      target: '../libs/contract/operations.ts',
      schemas: { path: '../libs/contract/schemas.ts', type: 'zod', mode: 'single' },
      client: 'zod',
      override: {
        zod: { version: 4, generateReusableSchemas: true },
      },
    },
  },
})
