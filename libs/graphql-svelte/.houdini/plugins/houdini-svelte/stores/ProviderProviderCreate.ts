import artifact from '$houdini/artifacts/ProviderProviderCreate.js'
import type { ProviderProviderCreate$result, ProviderProviderCreate$input, ProviderProviderCreate$optimistic } from '$houdini/artifacts/ProviderProviderCreate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderProviderCreateStore extends MutationStore<ProviderProviderCreate$result, ProviderProviderCreate$input, ProviderProviderCreate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
