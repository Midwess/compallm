export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Query = {
  __typename?: 'Query';
  /**
   * Read the selected login's verified identity and organization-name snapshot.
   *
   * Requires active, current user access. No credentials, policy source, or
   * freshness internals are exposed; no user/organization hydration is performed.
   */
  session: Session;
};

export type Session = {
  __typename?: 'Session';
  /** Login session identity, serialized as a string without numeric precision loss. */
  id: Scalars['ID']['output'];
  /** Organization selected for this verified session. */
  organizationId: Scalars['ID']['output'];
  /** Signed display snapshot; the name never establishes organization authority. */
  organizationName: Scalars['String']['output'];
  /** User owning this verified session. */
  userId: Scalars['ID']['output'];
};
