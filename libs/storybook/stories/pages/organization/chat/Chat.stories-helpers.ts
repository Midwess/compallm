import { createChatStore, type ChatController } from '../../../../src/pages/organization/chat'

export class DemoChatController implements ChatController {
  readonly store = createChatStore({
    modelId: 'fast',
    models: [
      { value: 'fast', label: 'Fast Chat' },
      { value: 'sonnet', label: 'claude-3.5-sonnet' },
    ],
  })
  constructor(private readonly report: (value: string) => void) {}
  onCreateChat = () => this.report('Create chat requested')
  onAttach = () => this.report('Attach file')
}
export { default as Demo } from './Chat.stories-Demo.svelte'
