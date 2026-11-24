import js from '@eslint/js'
import ts from 'typescript-eslint'
import svelte from 'eslint-plugin-svelte'
import globals from 'globals'
const svelteConfig = { compilerOptions: { experimental: { async: true } } }

export default ts.config(
  { ignores: ['**/node_modules/**', 'dist/**', '.svelte-kit/**', 'build/**', 'storybook-static/**'] },
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  ...svelte.configs.prettier,
  {
    linterOptions: { reportUnusedDisableDirectives: 'error' },
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
      parserOptions: { extraFileExtensions: ['.svelte'] },
    },
    rules: {
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_', destructuredArrayIgnorePattern: '^_' },
      ],
      'svelte/button-has-type': 'error',
      'svelte/no-at-debug-tags': 'error',
      'svelte/no-conflicting-module-names': 'error',
      'svelte/no-inspect': 'error',
      'svelte/valid-compile': 'error',
    },
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
    languageOptions: {
      parserOptions: { parser: ts.parser, projectService: true, svelteConfig },
    },
  },
)
