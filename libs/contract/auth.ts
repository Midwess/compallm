import { EmailInput, EmailCodeInput, EmailCodeInputConstraints, EmailCodeInputInput } from './validation.ts'

export const emailRequestSchema = (mode: 'signup' | 'signin') => ({ signup: EmailInput, signin: EmailInput })[mode]
export const otpRequestSchema = (mode: 'signup' | 'signin') =>
  ({ signup: EmailCodeInput, signin: EmailCodeInput })[mode]
export const otpLength = EmailCodeInputConstraints.code.maxLength
export const otpPattern = EmailCodeInputConstraints.code.pattern

export function otpCharacters(value: string): string {
  return [...value]
    .filter((character) => EmailCodeInputInput.code(character))
    .slice(0, otpLength)
    .join('')
}
