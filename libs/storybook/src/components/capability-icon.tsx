import type { ComponentProps } from 'react'
import type { ProviderModelCapability } from '@compallm/contract/graphql'
import { TextTIcon } from '@phosphor-icons/react/dist/csr/TextT'
import { ImageSquareIcon } from '@phosphor-icons/react/dist/csr/ImageSquare'
import { FilePdfIcon } from '@phosphor-icons/react/dist/csr/FilePdf'
import { MicrophoneIcon } from '@phosphor-icons/react/dist/csr/Microphone'
import { VideoCameraIcon } from '@phosphor-icons/react/dist/csr/VideoCamera'
import { SpeakerHighIcon } from '@phosphor-icons/react/dist/csr/SpeakerHigh'
import { GlobeIcon } from '@phosphor-icons/react/dist/csr/Globe'
import { BrainIcon } from '@phosphor-icons/react/dist/csr/Brain'
import { WrenchIcon } from '@phosphor-icons/react/dist/csr/Wrench'
import { BracketsCurlyIcon } from '@phosphor-icons/react/dist/csr/BracketsCurly'
import { SparkleIcon } from '@phosphor-icons/react/dist/csr/Sparkle'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { cx } from './shared'
import { Tooltip } from './feedback'

export type CapabilityKind = Lowercase<ProviderModelCapability>

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

const capabilityArtwork = {
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
} satisfies Record<CapabilityKind, { icon: typeof TextTIcon; tone: string }>

export function CapabilityIcon({
  capability,
  className,
  ...props
}: {
  capability: CapabilityKind
} & Omit<ComponentProps<'span'>, 'children'>) {
  const { icon: Icon, tone } = capabilityArtwork[capability]
  const adorned = capability === 'image_generation' || capability === 'forced_tool_choice'
  return (
    <span
      {...props}
      aria-hidden="true"
      data-capability={capability}
      className={cx('relative inline-flex size-4 shrink-0 items-center justify-center', tone, className)}
    >
      <Icon weight="regular" className={cx('size-full', adorned && 'scale-90 origin-bottom-left')} />
      {adorned ? (
        <span className="absolute -top-0.5 -right-0.5 flex h-1/2 w-1/2 items-center justify-center">
          {capability === 'image_generation' ? (
            <SparkleIcon weight="fill" className="size-full" />
          ) : (
            <CheckIcon weight="bold" className="size-full" />
          )}
        </span>
      ) : null}
    </span>
  )
}

export function CapabilityIndicator({ capability }: { capability: CapabilityKind }) {
  const label = capabilityLabels[capability]
  return (
    <Tooltip.Control content={label}>
      <span
        tabIndex={0}
        role="img"
        aria-label={label}
        className="inline-flex size-4 shrink-0 items-center justify-center rounded-sm outline-none
      focus-visible:outline-1 focus-visible:outline-ring"
      >
        <CapabilityIcon capability={capability} className="size-3" />
      </span>
    </Tooltip.Control>
  )
}
