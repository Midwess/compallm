import { useId, type SVGProps } from 'react'
import { ImageSquareIcon } from '@phosphor-icons/react/dist/csr/ImageSquare'
import googleAIStudioLogo from '../assets/provider-logos/GoogleAIStudioIcon.svg'
import { cx } from './shared'

export function OpenAIIcon({ className, ...props }: SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={props['aria-label'] ? undefined : true}
      className={cx('size-4 shrink-0', className)}
    >
      <g transform="translate(0.0 0.0) scale(1.0)" fill="currentColor" fillRule="evenodd">
        <path
          d={
            'M9.205 8.658v-2.26c0-.19.072-.333.238-.428l4.543-2.616c.619-.357 1.356-.523 2.117-.523 2.854 0 4' +
            '.662 2.212 4.662 4.566 0 .167 0 .357-.024.547l-4.71-2.759a.797.797 0 00-.856 0l-5.97 3.473zm10.6' +
            '09 8.8V12.06c0-.333-.143-.57-.429-.737l-5.97-3.473 1.95-1.118a.433.433 0 01.476 0l4.543 2.617c1.' +
            '309.76 2.189 2.378 2.189 3.948 0 1.808-1.07 3.473-2.76 4.163zM7.802 12.703l-1.95-1.142c-.167-.09' +
            '5-.239-.238-.239-.428V5.899c0-2.545 1.95-4.472 4.591-4.472 1 0 1.927.333 2.712.928L8.23 5.067c-.' +
            '285.166-.428.404-.428.737v6.898zM12 15.128l-2.795-1.57v-3.33L12 8.658l2.795 1.57v3.33L12 15.128z' +
            'm1.796 7.23c-1 0-1.927-.332-2.712-.927l4.686-2.712c.285-.166.428-.404.428-.737v-6.898l1.974 1.14' +
            '2c.167.095.238.238.238.428v5.233c0 2.545-1.974 4.472-4.614 4.472zm-5.637-5.303l-4.544-2.617c-1.3' +
            '08-.761-2.188-2.378-2.188-3.948A4.482 4.482 0 014.21 6.327v5.423c0 .333.143.571.428.738l5.947 3.' +
            '449-1.95 1.118a.432.432 0 01-.476 0zm-.262 3.9c-2.688 0-4.662-2.021-4.662-4.519 0-.19.024-.38.04' +
            '7-.57l4.686 2.71c.286.167.571.167.856 0l5.97-3.448v2.26c0 .19-.07.333-.237.428l-4.543 2.616c-.61' +
            '9.357-1.356.523-2.117.523zm5.899 2.83a5.947 5.947 0 005.827-4.756C22.287 18.339 24 15.84 24 13.2' +
            '96c0-1.665-.713-3.282-1.998-4.448.119-.5.19-.999.19-1.498 0-3.401-2.759-5.947-5.946-5.947-.642 0' +
            '-1.26.095-1.88.31A5.962 5.962 0 0010.205 0a5.947 5.947 0 00-5.827 4.757C1.713 5.447 0 7.945 0 10' +
            '.49c0 1.666.713 3.283 1.998 4.448-.119.5-.19 1-.19 1.499 0 3.401 2.759 5.946 5.946 5.946.642 0 1' +
            '.26-.095 1.88-.309a5.96 5.96 0 004.162 1.713z'
          }
        />
      </g>
    </svg>
  )
}

export function AnthropicIcon({ className, ...props }: SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={props['aria-label'] ? undefined : true}
      className={cx('size-4 shrink-0', className)}
    >
      <g transform="translate(0.0 3.7714285714285722) scale(0.6857142857142857)" fill="none">
        <path
          d={
            'M-0.00143743 24L9.56656 -1.90735e-06H15.0546L24.6226 24H19.2626L17.3106 18.96H7.29456L5.34256 24' +
            'H-0.00143743ZM9.02256 14.496H15.5826L12.3026 6.064L9.02256 14.496ZM29.0386 24L19.4706 -1.90735e-' +
            '06H24.7186L34.2866 24H29.0386Z'
          }
          fill="currentColor"
        />
      </g>
    </svg>
  )
}

export function GoogleAIStudioIcon({ className, ...props }: SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={props['aria-label'] ? undefined : true}
      className={cx('size-4 shrink-0', className)}
    >
      <image href={googleAIStudioLogo} width="24" height="24" />
    </svg>
  )
}

export function DeepSeekIcon({ className, ...props }: SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={props['aria-label'] ? undefined : true}
      className={cx('size-4 shrink-0', className)}
    >
      <g transform="translate(0.0 0.0) scale(1.0)" fill="currentColor" fillRule="evenodd">
        <path
          d={
            'M23.748 4.482c-.254-.124-.364.113-.512.234-.051.039-.094.09-.137.136-.372.397-.806.657-1.373.626' +
            '-.829-.046-1.537.214-2.163.848-.133-.782-.575-1.248-1.247-1.548-.352-.156-.708-.311-.955-.65-.17' +
            '2-.241-.219-.51-.305-.774-.055-.16-.11-.323-.293-.35-.2-.031-.278.136-.356.276-.313.572-.434 1.2' +
            '02-.422 1.84.027 1.436.633 2.58 1.838 3.393.137.093.172.187.129.323-.082.28-.18.552-.266.833-.05' +
            '5.179-.137.217-.329.14a5.526 5.526 0 01-1.736-1.18c-.857-.828-1.631-1.742-2.597-2.458a11.365 11.' +
            '365 0 00-.689-.471c-.985-.957.13-1.743.388-1.836.27-.098.093-.432-.779-.428-.872.004-1.67.295-2.' +
            '687.684a3.055 3.055 0 01-.465.137 9.597 9.597 0 00-2.883-.102c-1.885.21-3.39 1.102-4.497 2.623C.' +
            '082 8.606-.231 10.684.152 12.85c.403 2.284 1.569 4.175 3.36 5.653 1.858 1.533 3.997 2.284 6.438 ' +
            '2.14 1.482-.085 3.133-.284 4.994-1.86.47.234.962.327 1.78.397.63.059 1.236-.03 1.705-.128.735-.1' +
            '56.684-.837.419-.961-2.155-1.004-1.682-.595-2.113-.926 1.096-1.296 2.746-2.642 3.392-7.003.05-.3' +
            '47.007-.565 0-.845-.004-.17.035-.237.23-.256a4.173 4.173 0 001.545-.475c1.396-.763 1.96-2.015 2.' +
            '093-3.517.02-.23-.004-.467-.247-.588zM11.581 18c-2.089-1.642-3.102-2.183-3.52-2.16-.392.024-.321' +
            '.471-.235.763.09.288.207.486.371.739.114.167.192.416-.113.603-.673.416-1.842-.14-1.897-.167-1.36' +
            '1-.802-2.5-1.86-3.301-3.307-.774-1.393-1.224-2.887-1.298-4.482-.02-.386.093-.522.477-.592a4.696 ' +
            '4.696 0 011.529-.039c2.132.312 3.946 1.265 5.468 2.774.868.86 1.525 1.887 2.202 2.891.72 1.066 1' +
            '.494 2.082 2.48 2.914.348.292.625.514.891.677-.802.09-2.14.11-3.054-.614zm1-6.44a.306.306 0 01.4' +
            '15-.287.302.302 0 01.2.288.306.306 0 01-.31.307.303.303 0 01-.304-.308zm3.11 1.596c-.2.081-.399.' +
            '151-.59.16a1.245 1.245 0 01-.798-.254c-.274-.23-.47-.358-.552-.758a1.73 1.73 0 01.016-.588c.07-.' +
            '327-.008-.537-.239-.727-.187-.156-.426-.199-.688-.199a.559.559 0 01-.254-.078c-.11-.054-.2-.19-.' +
            '114-.358.028-.054.16-.186.192-.21.356-.202.767-.136 1.146.016.352.144.618.408 1.001.782.391.451.' +
            '462.576.685.914.176.265.336.537.445.848.067.195-.019.354-.25.452z'
          }
        />
      </g>
    </svg>
  )
}

export function GroqIcon({ className, ...props }: SVGProps<SVGSVGElement>) {
  const id = useId()
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={props['aria-label'] ? undefined : true}
      className={cx('size-4 shrink-0', className)}
    >
      <g transform="translate(0.0 0.0) scale(0.7272727272727273)" fill="none">
        <g clipPath={`url(#${id}-groq-a)`}>
          <path fill="#F43E01" d="M.54.39h32v32h-32z" />
          <path fill="#fff" d="m18.445 4.406-9.468 13.74 7.341.665-1.69 9.578 9.469-13.74-7.342-.664 1.69-9.579Z" />
        </g>
        <defs>
          <clipPath id={`${id}-groq-a`}>
            <path fill="#fff" d="M.54.39h32v32h-32z" />
          </clipPath>
        </defs>
      </g>
    </svg>
  )
}

export function MistralIcon({ className, ...props }: SVGProps<SVGSVGElement>) {
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      aria-hidden={props['aria-label'] ? undefined : true}
      className={cx('size-4 shrink-0', className)}
    >
      <g transform="translate(0.0 3.428571428571429) scale(1.1428571428571428)" fill="none">
        <path d="M6 0H3V3H6V0Z" fill="#FFAF01" />
        <path d="M6 9H3V12H6V9Z" fill="#E61300" />
        <path d="M9 12H0V15H9V12Z" fill="#C4001D" />
        <path d="M9 3H3V6H9V3Z" fill="#FF8204" />
        <path d="M12 9H9V12H12V9Z" fill="#E61300" />
        <path d="M18 6H3V9H18V6Z" fill="#FA500F" />
        <path d="M18 3H12V6H18V3Z" fill="#FF8204" />
        <path d="M18 0H15V3H18V0Z" fill="#FFAF01" />
        <path d="M18 9H15V12H18V9Z" fill="#E61300" />
        <path d="M21 12H12V15H21V12Z" fill="#C4001D" />
      </g>
    </svg>
  )
}

export type ProviderBrand = 'openai' | 'anthropic' | 'gemini' | 'deepseek' | 'groq' | 'mistral'
const logos = {
  openai: OpenAIIcon,
  anthropic: AnthropicIcon,
  gemini: GoogleAIStudioIcon,
  deepseek: DeepSeekIcon,
  groq: GroqIcon,
  mistral: MistralIcon,
}
export function providerBrand(value: string | null | undefined): ProviderBrand | undefined {
  const name = value?.toLowerCase() ?? ''
  return (
    (Object.keys(logos) as ProviderBrand[]).find((brand) => name.includes(brand)) ??
    (name.includes('google') ? 'gemini' : undefined)
  )
}
export function ProviderIcon({ provider, ...props }: SVGProps<SVGSVGElement> & { provider?: string | null }) {
  const brand = providerBrand(provider)
  const Icon = brand ? logos[brand] : ImageSquareIcon
  return (
    <Icon {...props} aria-hidden={props['aria-label'] ? undefined : true} className={cx('size-4 shrink-0', props.className)} />
  )
}
