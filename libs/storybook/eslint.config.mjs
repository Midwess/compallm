import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import tseslint from 'typescript-eslint'

export default tseslint.config(
  { ignores: ['dist/**', 'storybook-static/**', 'node_modules/**', 'test-results/**', 'playwright-report/**'] },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['**/*.{ts,tsx,mjs}'],
    languageOptions: { globals: { ...globals.browser, ...globals.node } },
    plugins: { 'react-hooks': reactHooks },
    rules: {
      ...reactHooks.configs.recommended.rules,
      'max-len': ['error', { code: 128, tabWidth: 2 }],
    },
  },
  {
    files: ['src/components/**/*.{ts,tsx}', 'src/pages/**/*.{ts,tsx}', 'stories/components/**/*.{ts,tsx}'],
    rules: {
      'no-restricted-imports': [
        'error',
        {
          patterns: [
            { group: ['*.css', '**/*.css'], message: 'Use Tailwind utilities; load CSS only in Tailwind entry points.' },
          ],
        },
      ],
    },
  },
)
