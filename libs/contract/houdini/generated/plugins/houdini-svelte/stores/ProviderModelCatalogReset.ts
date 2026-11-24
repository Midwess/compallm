import artifact from '$houdini/artifacts/ProviderModelCatalogReset.js'
import type { ProviderModelCatalogReset$result, ProviderModelCatalogReset$input, ProviderModelCatalogReset$optimistic } from '$houdini/artifacts/ProviderModelCatalogReset.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelCatalogResetStore extends MutationStore<ProviderModelCatalogReset$result, ProviderModelCatalogReset$input, ProviderModelCatalogReset$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
