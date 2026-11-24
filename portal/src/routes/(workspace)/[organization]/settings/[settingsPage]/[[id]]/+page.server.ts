import { settingsLoad } from '../../../../../../lib/server/loads'
import type { PageServerLoad } from './$types'
export const load: PageServerLoad = settingsLoad
