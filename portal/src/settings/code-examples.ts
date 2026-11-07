import type { SettingsState } from '@compallm/storybook/pages/settings'

// Static preview content, independent of workspace records and API contracts.
export const codeExamples: SettingsState['codeExamples'] = {
  typescript: {
    code: [
      'interface Greeting {',
      '  message: string',
      '}',
      '',
      '// Create a friendly greeting.',
      'function greet(name: string): Greeting {',
      '  return { message: `Hello, ${name}!` }',
      '}',
      '',
      'const greeting = greet("Alex")',
      'console.log(greeting.message)',
    ].join('\n'),
  },
  javascript: {
    code: '// Create a friendly greeting.\nfunction greet(name) {\n  return `Hello, ${name}!`\n}\n\nconsole.log(greet("Alex"))',
  },
  json: { code: '{\n  "message": "Hello, Alex!",\n  "enabled": true,\n  "count": 3\n}' },
  bash: { code: '# Print a greeting.\nname="Alex"\nprintf "Hello, %s!\\n" "$name"' },
  cedar: { code: '// Example policy syntax.\npermit(\n  principal,\n  action,\n  resource\n);' },
}
