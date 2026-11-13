import type { PreferenceRelativeFontSize } from '@compallm/contract/graphql'

export class AppearanceController {
  setTextSize = (size: PreferenceRelativeFontSize) => {
    if (typeof document !== 'undefined') document.documentElement.dataset.textSize = size.toLowerCase()
  }
}

export const appearanceController = new AppearanceController()
