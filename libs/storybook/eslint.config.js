import js from '@eslint/js'
import ts from 'typescript-eslint'
import svelte from 'eslint-plugin-svelte'
import globals from 'globals'
import svelteConfig from './svelte.config.js'

export default ts.config(
  { ignores: ['**/node_modules/**', 'dist/**', '.svelte-kit/**', 'storybook-static/**'] },
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
    files: ['src/state.svelte.ts', 'src/components/form-model.svelte.ts', 'src/components/native-form.svelte.ts'],
    // Listener and submission registries are imperative; values and visible status use native reactivity explicitly.
    rules: { 'svelte/prefer-svelte-reactivity': 'off' },
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
    languageOptions: {
      parserOptions: { parser: ts.parser, projectService: true, svelteConfig },
    },
  },
)
