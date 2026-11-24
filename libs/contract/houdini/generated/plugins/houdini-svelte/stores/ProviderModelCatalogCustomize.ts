import artifact from '$houdini/artifacts/ProviderModelCatalogCustomize.js'
import type { ProviderModelCatalogCustomize$result, ProviderModelCatalogCustomize$input, ProviderModelCatalogCustomize$optimistic } from '$houdini/artifacts/ProviderModelCatalogCustomize.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelCatalogCustomizeStore extends MutationStore<ProviderModelCatalogCustomize$result, ProviderModelCatalogCustomize$input, ProviderModelCatalogCustomize$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
