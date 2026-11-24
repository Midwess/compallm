import { graphqlClient } from './graphql-client'
export const requestActivity = graphqlClient.activity
export function beginRequest() {
  let finished = false
  requestActivity.setState((state) => ({ pending: state.pending + 1 }))
  return () => {
    if (!finished) {
      finished = true
      requestActivity.setState((state) => ({ pending: state.pending - 1 }))
    }
  }
}
