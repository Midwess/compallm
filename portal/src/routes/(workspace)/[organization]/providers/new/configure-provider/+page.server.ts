import { providerLoad } from '../../../../../../lib/server/loads'
import type { PageServerLoad } from './$types'
export const load: PageServerLoad = (event) => providerLoad(event, undefined)
