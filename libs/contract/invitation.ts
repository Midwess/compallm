import { z } from 'zod'
import { InvitationBatchCreateInput, InvitationCreateInput } from './validation.js'

export function splitInvitationEmails(text: string): string[] {
  return [...new Set(text.trim().split(/[\s;,|]+/u).filter(Boolean).map((email) => {
    const value = email.trim()
    const parsed = InvitationCreateInput.safeParse({ email: value })
    return parsed.success ? parsed.data.email : value
  }))]
}

export const invitationTextSchema = z.object({
  email: z.string().superRefine((text, context) => {
    const result = InvitationBatchCreateInput.safeParse({ invitations: splitInvitationEmails(text).map((email) => ({ email })) })
    if (!result.success) for (const issue of result.error.issues) context.addIssue({ code: 'custom', message: issue.message })
  }).transform((text) => splitInvitationEmails(text).join('\n')),
})

export function invitationBatchInput(text: string) {
  return InvitationBatchCreateInput.parse({ invitations: splitInvitationEmails(text).map((email) => ({ email })) })
}
