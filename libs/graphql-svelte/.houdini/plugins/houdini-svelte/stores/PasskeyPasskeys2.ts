import type { QueryStoreFetchParams } from '$houdini'
import { QueryStore } from '$houdini/plugins/houdini-svelte/runtime/stores/query.js'
import artifact from '$houdini/artifacts/PasskeyPasskeys2.js'
import type { PasskeyPasskeys2$result, PasskeyPasskeys2$input } from '$houdini/artifacts/PasskeyPasskeys2.js'

export class PasskeyPasskeys2Store extends QueryStore<PasskeyPasskeys2$result, PasskeyPasskeys2$input, typeof artifact> {
    constructor() {
        super({
            artifact,
            storeName: "PasskeyPasskeys2Store",
            variables: false,
        })
    }
}

export async function load_PasskeyPasskeys2(params: QueryStoreFetchParams<PasskeyPasskeys2$result, PasskeyPasskeys2$input>): Promise<{PasskeyPasskeys2: PasskeyPasskeys2Store}>{
    const store = new PasskeyPasskeys2Store()
    await store.fetch(params)
    return { PasskeyPasskeys2: store }
}
