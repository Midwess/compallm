import { HoudiniClient } from '../.houdini/runtime/client'

export default new HoudiniClient({ fetchParams: () => ({ credentials: 'same-origin' }) })
