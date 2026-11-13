import { createChatStore, type ChatController as Controller } from '@compallm/storybook/pages/organization/chat'
import { notify } from '../notifications/toast'

export class ChatController implements Controller {
  readonly store = createChatStore()
  onCreateChat = () => {
    notify.info('Chat creation is not available yet.')
    return false as const
  }
  onAttach = () => {
    notify.info('Attachments are not available yet.')
  }
}
