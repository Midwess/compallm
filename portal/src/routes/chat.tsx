import { useState } from 'react'
import { ChatControllerContext } from '@compallm/storybook/context'
import { ChatPage } from '@compallm/storybook/pages/organization/chat'
import { ChatController } from '../chat/controller'

export function Component() {
  const [controller] = useState(() => new ChatController())
  return (
    <ChatControllerContext value={controller}>
      <ChatPage />
    </ChatControllerContext>
  )
}
