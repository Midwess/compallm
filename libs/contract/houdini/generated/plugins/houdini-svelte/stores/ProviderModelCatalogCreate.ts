import artifact from '$houdini/artifacts/ProviderModelCatalogCreate.js'
import type { ProviderModelCatalogCreate$result, ProviderModelCatalogCreate$input, ProviderModelCatalogCreate$optimistic } from '$houdini/artifacts/ProviderModelCatalogCreate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelCatalogCreateStore extends MutationStore<ProviderModelCatalogCreate$result, ProviderModelCatalogCreate$input, ProviderModelCatalogCreate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
