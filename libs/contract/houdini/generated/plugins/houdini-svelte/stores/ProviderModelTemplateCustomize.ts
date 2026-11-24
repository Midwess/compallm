import artifact from '$houdini/artifacts/ProviderModelTemplateCustomize.js'
import type { ProviderModelTemplateCustomize$result, ProviderModelTemplateCustomize$input, ProviderModelTemplateCustomize$optimistic } from '$houdini/artifacts/ProviderModelTemplateCustomize.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderModelTemplateCustomizeStore extends MutationStore<ProviderModelTemplateCustomize$result, ProviderModelTemplateCustomize$input, ProviderModelTemplateCustomize$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
