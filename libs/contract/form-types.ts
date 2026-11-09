// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.

import type { z } from 'zod'
import * as schemas from './validation.ts'
/** How the request delivers credentials; this is never stored in signed claims. */
export type CredentialDelivery = 'COOKIE' | 'TOKEN'
export const CredentialDelivery = schemas.CredentialDelivery.enum
/** Shared input for confirming a signup or signin email challenge.

The verified encrypted challenge comes from request context, never this input.
The adapter calls `validify()` before Core verifies the code and challenge. */
export interface EmailCodeInput {
  /** Exactly six ASCII digits after trimming surrounding whitespace. */
  code: string
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery
}
/** Shared input for requesting either a signup or a signin email challenge.

Call `validify()` before Core execution. GraphQL parsing alone does not run
normalization, application validation, or email ownership checks. */
export interface EmailInput {
  /** Trimmed and ASCII-lowercased using the shared Rust email policy. */
  email: string
  /** Cookie delivery is the default; token delivery must be requested explicitly. */
  delivery?: CredentialDelivery
}
export type SignupEmailRequest = z.input<typeof schemas.SignupEmailRequest>
export type SigninEmailRequest = z.input<typeof schemas.SigninEmailRequest>
export type ConfirmSignupOtpRequest = z.input<typeof schemas.ConfirmSignupOtpRequest>
export type ConfirmSigninOtpRequest = z.input<typeof schemas.ConfirmSigninOtpRequest>
