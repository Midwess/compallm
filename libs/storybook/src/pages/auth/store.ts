import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'
import { createForm, type FormOptions } from '../../components/form-model.svelte'
import { createStore } from '../../state.svelte'
import type { AuthController, AuthDraft, AuthMode, AuthState } from './types'

export function createAuthStore(initial: Partial<AuthState> = {}) {
  return createStore<AuthState>(() => ({
    currentUser: null,
    status: 'unknown',
    step: 'email',
    email: '',
    error: null,
    serverError: null,
    pending: null,
    remaining: 0,
    lastLoginMethod: null,
    verificationActive: false,
    ...initial,
  }))
}
export function createAuthForm(
  controller: Pick<AuthController, 'store' | 'requestCode' | 'verifyCode' | 'clearError'>,
  mode: AuthMode,
  initial: Partial<AuthDraft> = {},
) {
  return createForm<AuthDraft>(
    { email: controller.store.getState().email, code: '', ...initial },
    authFormOptions(controller, mode),
  )
}
export function authFormOptions(
  controller: Pick<AuthController, 'store' | 'requestCode' | 'verifyCode' | 'clearError'>,
  mode: AuthMode,
): FormOptions<AuthDraft> {
  return {
    mode: 'button',
    command: true,
    changed: () => controller.clearError(),
    parse(values) {
      if (controller.store.getState().step === 'code') {
        const result = otpRequestSchema(mode).safeParse({ code: values.code })
        return result.success ? { values: { ...values, code: result.data.code } } : { issues: result.error.issues }
      }
      const result = emailRequestSchema(mode).safeParse({ email: values.email })
      return result.success ? { values: { ...values, email: result.data.email } } : { issues: result.error.issues }
    },
    async submit(values) {
      if (controller.store.getState().step === 'code') {
        await controller.verifyCode(values.code)
        return controller.store.getState().step === 'complete' ? values : false
      }
      await controller.requestCode(values.email)
      return controller.store.getState().step === 'code' ? { email: controller.store.getState().email } : false
    },
  }
}
