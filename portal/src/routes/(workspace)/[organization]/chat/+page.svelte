<script lang="ts">
  import { ChatPage } from '@compallm/storybook/pages/organization/chat'
  import { setChatController } from '@compallm/storybook/context'
  import { ChatController } from '../../../../chat/controller'
  setChatController(new ChatController())
</script>

<ChatPage />
