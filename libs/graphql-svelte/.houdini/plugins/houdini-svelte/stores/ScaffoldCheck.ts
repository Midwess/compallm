import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/ScaffoldCheck.js'
import type { ScaffoldCheck$result, ScaffoldCheck$input } from '$houdini/artifacts/ScaffoldCheck.js'

export class ScaffoldCheckStore extends QueryStore<ScaffoldCheck$result, ScaffoldCheck$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "ScaffoldCheckStore",
            variables: false,
        })
    }
}

export async function load_ScaffoldCheck(params: QueryStoreFetchParams<ScaffoldCheck$result, ScaffoldCheck$input>): Promise<{ScaffoldCheck: ScaffoldCheckStore}>{
    const store = new ScaffoldCheckStore()
    await store.fetch(params)
    return { ScaffoldCheck: store }
}
