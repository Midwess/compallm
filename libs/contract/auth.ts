import {
  SignupEmailRequest,
  SigninEmailRequest,
  ConfirmSignupOtpRequest,
  ConfirmSigninOtpRequest,
  ConfirmSignupOtpRequestConstraints,
  ConfirmSignupOtpRequestInput,
} from './validation.ts'

export const emailRequestSchema = (mode: 'signup' | 'signin') => (mode === 'signup' ? SignupEmailRequest : SigninEmailRequest)
export const otpRequestSchema = (mode: 'signup' | 'signin') =>
  mode === 'signup' ? ConfirmSignupOtpRequest : ConfirmSigninOtpRequest
export const otpLength = ConfirmSignupOtpRequestConstraints.code.maxLength
export const otpPattern = ConfirmSignupOtpRequestConstraints.code.pattern

export function otpCharacters(value: string): string {
  return [...value]
    .filter((character) => ConfirmSignupOtpRequestInput.code(character))
    .slice(0, otpLength)
    .join('')
}
