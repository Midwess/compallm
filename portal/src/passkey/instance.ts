import { PasskeyController } from './controller'
import { authController } from '../auth/controller'
export const passkeyController = new PasskeyController(authController)
