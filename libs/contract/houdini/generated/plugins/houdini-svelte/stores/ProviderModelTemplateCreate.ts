import artifact from '$houdini/artifacts/ProviderModelTemplateCreate.js'
import type { ProviderModelTemplateCreate$result, ProviderModelTemplateCreate$input, ProviderModelTemplateCreate$optimistic } from '$houdini/artifacts/ProviderModelTemplateCreate.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelTemplateCreateStore extends MutationStore<ProviderModelTemplateCreate$result, ProviderModelTemplateCreate$input, ProviderModelTemplateCreate$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
