import { createChatStore, type ChatController as Controller } from '@compallm/storybook/pages/organization/chat'
import some from 'lodash-es/some'
import { notify } from '../notifications/toast'

export class ChatController implements Controller {
  readonly store = createChatStore()
  onDraftChanged = (draft: string) => this.store.setState({ draft })
  onModelChanged = (modelId: string) => {
    if (some(this.store.getState().models, { value: modelId })) this.store.setState({ modelId })
  }
  onCreateChat = () => {
    notify.info('Chat creation is not available yet.')
  }
  onAttach = () => {
    notify.info('Attachments are not available yet.')
  }
}
