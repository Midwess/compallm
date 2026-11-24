import artifact from '$houdini/artifacts/ProviderModelCatalogDelete.js'
import type { ProviderModelCatalogDelete$result, ProviderModelCatalogDelete$input, ProviderModelCatalogDelete$optimistic } from '$houdini/artifacts/ProviderModelCatalogDelete.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelCatalogDeleteStore extends MutationStore<ProviderModelCatalogDelete$result, ProviderModelCatalogDelete$input, ProviderModelCatalogDelete$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
