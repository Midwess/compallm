import type { Content } from '../../components/shared'

export interface MainPageProps {
  sidebar: Content
  children?: Content
  contentLabel?: string
}
export { default as MainPage } from './index-MainPage.svelte'
