import artifact from '$houdini/artifacts/PasskeyPasskeyRegistrationStart.js'
import type { PasskeyPasskeyRegistrationStart$result, PasskeyPasskeyRegistrationStart$input, PasskeyPasskeyRegistrationStart$optimistic } from '$houdini/artifacts/PasskeyPasskeyRegistrationStart.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class PasskeyPasskeyRegistrationStartStore extends MutationStore<PasskeyPasskeyRegistrationStart$result, PasskeyPasskeyRegistrationStart$input, PasskeyPasskeyRegistrationStart$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
