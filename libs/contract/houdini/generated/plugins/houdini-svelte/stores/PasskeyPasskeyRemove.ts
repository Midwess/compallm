import artifact from '$houdini/artifacts/PasskeyPasskeyRemove.js'
import type { PasskeyPasskeyRemove$result, PasskeyPasskeyRemove$input, PasskeyPasskeyRemove$optimistic } from '$houdini/artifacts/PasskeyPasskeyRemove.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class PasskeyPasskeyRemoveStore extends MutationStore<PasskeyPasskeyRemove$result, PasskeyPasskeyRemove$input, PasskeyPasskeyRemove$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
