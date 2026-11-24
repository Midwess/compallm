import { browser } from '$app/env'
import { graphqlClient } from '../../../../../../graphql-client'
import type { PageLoad } from './$types'
export const load: PageLoad = ({ data }) => ({
  ...data,
  settingsRead: data.settingsRead
    ? {
        ...data.settingsRead,
        version:
          !browser || graphqlClient.acceptsPreparation(data.settingsRead.preparationVersion) ? graphqlClient.readVersion : -1,
      }
    : undefined,
})
