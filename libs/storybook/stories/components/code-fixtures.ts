export function actionSource(action: string) {
  const condition = action === 'usage.read.own' ? ' when { resource.owner == principal }' : ''
  return `permit(\n  principal in ?principal,\n  action == Compallm::Action::${JSON.stringify(action)},\n  resource in ?resource\n)${condition};`
}
export const modelOperatorSource = `${actionSource('model.use')}\n\n${actionSource('usage.read.own')}`
export const typescriptExample = {
  code:
    'const config = {\n  apiUrl: "https://api.example.com",\n  timeout: 5000,\n  debug: true,\n' +
    '  headers: { "Content-Type": "application/json" },\n};\n' +
    'async function fetchUser(id: string): Promise<User> {\n  const url = `${config.apiUrl}/users/${id}`;\n' +
    'async function fetchUser(id: string): Promise<User | null> {\n  const url = `${config.apiUrl}/v2/users/${id}`;\n' +
    '  const res = await fetch(url);\n  return res.ok ? res.json() : null;',
  lineChanges: { 4: 'removed', 5: 'added', 7: 'removed', 8: 'removed', 9: 'added', 10: 'added', 12: 'added' } as const,
}
