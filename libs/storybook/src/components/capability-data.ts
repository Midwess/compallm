import TextTIcon from 'phosphor-svelte/lib/TextTIcon'
import ImageSquareIcon from 'phosphor-svelte/lib/ImageSquareIcon'
import FilePdfIcon from 'phosphor-svelte/lib/FilePdfIcon'
import MicrophoneIcon from 'phosphor-svelte/lib/MicrophoneIcon'
import VideoCameraIcon from 'phosphor-svelte/lib/VideoCameraIcon'
import SpeakerHighIcon from 'phosphor-svelte/lib/SpeakerHighIcon'
import GlobeIcon from 'phosphor-svelte/lib/GlobeIcon'
import BrainIcon from 'phosphor-svelte/lib/BrainIcon'
import WrenchIcon from 'phosphor-svelte/lib/WrenchIcon'
import BracketsCurlyIcon from 'phosphor-svelte/lib/BracketsCurlyIcon'
import type { ProviderModelCapability$options } from '@compallm/contract/houdini'
export type CapabilityKind = Lowercase<ProviderModelCapability$options>
export const capabilityLabels: Record<CapabilityKind, string> = {
  text: 'Text',
  vision: 'Vision',
  pdf: 'PDF',
  audio: 'Audio input',
  video: 'Video input',
  image_generation: 'Image generation',
  audio_generation: 'Audio generation',
  search: 'Web search',
  reasoning: 'Reasoning',
  tool_call: 'Tool calling',
  forced_tool_choice: 'Forced tool choice',
  structured_output: 'Structured output',
}

export const capabilityArtwork = {
  text: { icon: TextTIcon, tone: 'text-[color-mix(in_srgb,var(--ring)_78%,var(--foreground))]' },
  vision: { icon: ImageSquareIcon, tone: 'text-[color-mix(in_srgb,var(--success)_45%,var(--ring))]' },
  pdf: { icon: FilePdfIcon, tone: 'text-[color-mix(in_srgb,var(--destructive)_85%,var(--foreground))]' },
  audio: { icon: MicrophoneIcon, tone: 'text-[color-mix(in_srgb,var(--success)_82%,var(--foreground))]' },
  video: { icon: VideoCameraIcon, tone: 'text-[color-mix(in_srgb,var(--warning)_85%,var(--foreground))]' },
  image_generation: { icon: ImageSquareIcon, tone: 'text-[color-mix(in_srgb,var(--primary)_50%,var(--destructive))]' },
  audio_generation: { icon: SpeakerHighIcon, tone: 'text-[color-mix(in_srgb,var(--destructive)_45%,var(--ring))]' },
  search: { icon: GlobeIcon, tone: 'text-[color-mix(in_srgb,var(--ring)_65%,var(--success))]' },
  reasoning: { icon: BrainIcon, tone: 'text-[color-mix(in_srgb,var(--primary)_70%,var(--foreground))]' },
  tool_call: { icon: WrenchIcon, tone: 'text-[color-mix(in_srgb,var(--warning)_70%,var(--success))]' },
  forced_tool_choice: { icon: WrenchIcon, tone: 'text-[color-mix(in_srgb,var(--warning)_55%,var(--destructive))]' },
  structured_output: { icon: BracketsCurlyIcon, tone: 'text-[color-mix(in_srgb,var(--success)_60%,var(--primary))]' },
}
