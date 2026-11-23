import artifact from '$houdini/artifacts/ProviderModelTemplateDelete.js'
import type { ProviderModelTemplateDelete$result, ProviderModelTemplateDelete$input, ProviderModelTemplateDelete$optimistic } from '$houdini/artifacts/ProviderModelTemplateDelete.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelTemplateDeleteStore extends MutationStore<ProviderModelTemplateDelete$result, ProviderModelTemplateDelete$input, ProviderModelTemplateDelete$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
