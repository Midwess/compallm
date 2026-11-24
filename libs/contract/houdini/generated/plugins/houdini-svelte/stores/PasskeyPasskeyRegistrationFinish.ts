import artifact from '$houdini/artifacts/PasskeyPasskeyRegistrationFinish.js'
import type { PasskeyPasskeyRegistrationFinish$result, PasskeyPasskeyRegistrationFinish$input, PasskeyPasskeyRegistrationFinish$optimistic } from '$houdini/artifacts/PasskeyPasskeyRegistrationFinish.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class PasskeyPasskeyRegistrationFinishStore extends MutationStore<PasskeyPasskeyRegistrationFinish$result, PasskeyPasskeyRegistrationFinish$input, PasskeyPasskeyRegistrationFinish$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
