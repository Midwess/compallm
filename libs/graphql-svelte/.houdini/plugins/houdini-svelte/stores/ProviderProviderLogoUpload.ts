import artifact from '$houdini/artifacts/ProviderProviderLogoUpload.js'
import type { ProviderProviderLogoUpload$result, ProviderProviderLogoUpload$input, ProviderProviderLogoUpload$optimistic } from '$houdini/artifacts/ProviderProviderLogoUpload.js'
import { MutationStore } from '$houdini/plugins/houdini-svelte/runtime/stores/mutation.js'

export class ProviderProviderLogoUploadStore extends MutationStore<ProviderProviderLogoUpload$result, ProviderProviderLogoUpload$input, ProviderProviderLogoUpload$optimistic> {
    constructor() {
        super({
            artifact,
        })
    }
}
