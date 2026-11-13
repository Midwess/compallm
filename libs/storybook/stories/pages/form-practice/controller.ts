import { createStore } from 'zustand/vanilla'
import {
  generateMutationOp,
  type MutationGenqlSelection,
  type Role,
  type RoleUpdateInput as Input,
} from '@compallm/contract/graphql'
import { RoleUpdateInput } from '@compallm/contract/validation'

export const initial: Pick<Role, 'id' | 'name' | 'description'> = {
  id: 'practice-role',
  name: 'Model operator',
  description: 'Explore models with the team.',
}

export class FormPracticeController {
  readonly store = createStore(() => ({
    confirmed: { ...initial },
    slow: true,
    rejectNext: false,
    request: null as ReturnType<typeof generateMutationOp> | null,
    response: null as { data: { roleUpdate: { node: typeof initial } } } | null,
  }))
  setSlow = (slow: boolean) => this.store.setState({ slow })
  reject = () => this.store.setState({ rejectNext: !this.store.getState().rejectNext })
  update = (patch: Pick<Input, 'name' | 'description'>, signal: AbortSignal): Promise<typeof initial> => {
    const state = this.store.getState()
    const selection = {
      roleUpdate: { __args: { id: initial.id, input: patch }, node: { id: true, name: true, description: true } },
    } satisfies MutationGenqlSelection
    this.store.setState({ request: generateMutationOp(selection), response: null, rejectNext: false })
    return new Promise((resolve, reject) => {
      if (signal.aborted) {
        reject(new Error('Cancelled'))
        return
      }
      const abort = () => {
        clearTimeout(timer)
        reject(new Error('Cancelled'))
      }
      const timer = setTimeout(
        () => {
          signal.removeEventListener('abort', abort)
          if (state.rejectNext) {
            reject(new Error('Mock write rejected'))
            return
          }
          const parsed = RoleUpdateInput.safeParse(patch)
          if (!parsed.success) {
            reject(new Error('Invalid update'))
            return
          }
          const node = { ...this.store.getState().confirmed }
          if (typeof parsed.data.name === 'string') node.name = parsed.data.name
          if (typeof parsed.data.description === 'string') node.description = parsed.data.description
          this.store.setState({ confirmed: node, response: { data: { roleUpdate: { node } } } })
          resolve(node)
        },
        state.slow ? 3000 : 500,
      )
      signal.addEventListener('abort', abort, { once: true })
    })
  }
}
