// @ts-nocheck
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

export type Scalars = {
  Boolean: boolean
  ID: string
  String: string
}

export interface Query {
  /**
   * Read the selected login's verified identity and organization-name snapshot.
   *
   * Requires active, current user access. No credentials, policy source, or
   * freshness internals are exposed; no user/organization hydration is performed.
   */
  session: Session
  __typename: 'Query'
}

export interface Session {
  /** Login session identity, serialized as a string without numeric precision loss. */
  id: Scalars['ID']
  /** Organization selected for this verified session. */
  organizationId: Scalars['ID']
  /** Signed display snapshot; the name never establishes organization authority. */
  organizationName: Scalars['String']
  /** User owning this verified session. */
  userId: Scalars['ID']
  __typename: 'Session'
}

export interface QueryGenqlSelection {
  /**
   * Read the selected login's verified identity and organization-name snapshot.
   *
   * Requires active, current user access. No credentials, policy source, or
   * freshness internals are exposed; no user/organization hydration is performed.
   */
  session?: SessionGenqlSelection
  __typename?: boolean | number
  __scalar?: boolean | number
}

export interface SessionGenqlSelection {
  /** Login session identity, serialized as a string without numeric precision loss. */
  id?: boolean | number
  /** Organization selected for this verified session. */
  organizationId?: boolean | number
  /** Signed display snapshot; the name never establishes organization authority. */
  organizationName?: boolean | number
  /** User owning this verified session. */
  userId?: boolean | number
  __typename?: boolean | number
  __scalar?: boolean | number
}

const Query_possibleTypes: string[] = ['Query']
export const isQuery = (obj?: { __typename?: any } | null): obj is Query => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isQuery"')
  return Query_possibleTypes.includes(obj.__typename)
}

const Session_possibleTypes: string[] = ['Session']
export const isSession = (obj?: { __typename?: any } | null): obj is Session => {
  if (!obj?.__typename) throw new Error('__typename is missing in "isSession"')
  return Session_possibleTypes.includes(obj.__typename)
}
