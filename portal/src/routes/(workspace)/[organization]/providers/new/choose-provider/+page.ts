import { browser } from '$app/env'
import { graphqlClient } from '../../../../../../graphql-client'
import type { PageLoad } from './$types'
export const load: PageLoad = ({ data }) => ({
  ...data,
  providerRead: data.providerRead
    ? {
        ...data.providerRead,
        version:
          !browser || graphqlClient.acceptsPreparation(data.providerRead.preparationVersion) ? graphqlClient.readVersion : -1,
      }
    : undefined,
})
