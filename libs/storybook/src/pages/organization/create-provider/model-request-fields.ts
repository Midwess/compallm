export const headerOptions = [
  { key: 'OpenAI-Project', description: 'Project identifier' },
  { key: 'OpenAI-Organization', description: 'Organization identifier' },
  { key: 'X-Client-Request-Id', description: 'Request correlation identifier' },
  { key: 'anthropic-beta', description: 'Model beta features' },
]
export const bodyOptions = [
  { key: 'temperature', description: 'Number · Response randomness' },
  { key: 'top_p', description: 'Number · Nucleus sampling' },
  { key: 'max_completion_tokens', description: 'Integer · Output-token budget' },
  { key: 'stop', description: 'String list · Stop sequences' },
  { key: 'seed', description: 'Integer · Reproducible sampling' },
  { key: 'presence_penalty', description: 'Number · Encourage new topics' },
  { key: 'frequency_penalty', description: 'Number · Reduce repetition' },
  { key: 'response_format', description: 'Object · Response structure' },
  { key: 'parallel_tool_calls', description: 'Boolean · Parallel tool calls' },
]
export { default as ModelRequestFields } from './model-request-fields-ModelRequestFields.svelte'
