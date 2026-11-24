import artifact from '$houdini/artifacts/PasskeyPasskeyUpdate.js'
import type { PasskeyPasskeyUpdate$result, PasskeyPasskeyUpdate$input, PasskeyPasskeyUpdate$optimistic } from '$houdini/artifacts/PasskeyPasskeyUpdate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class PasskeyPasskeyUpdateStore extends MutationStore<PasskeyPasskeyUpdate$result, PasskeyPasskeyUpdate$input, PasskeyPasskeyUpdate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
