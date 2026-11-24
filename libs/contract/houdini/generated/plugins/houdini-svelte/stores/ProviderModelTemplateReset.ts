import artifact from '$houdini/artifacts/ProviderModelTemplateReset.js'
import type { ProviderModelTemplateReset$result, ProviderModelTemplateReset$input, ProviderModelTemplateReset$optimistic } from '$houdini/artifacts/ProviderModelTemplateReset.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelTemplateResetStore extends MutationStore<ProviderModelTemplateReset$result, ProviderModelTemplateReset$input, ProviderModelTemplateReset$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
