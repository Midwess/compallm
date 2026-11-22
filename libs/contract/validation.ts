// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.

import { z } from 'zod'
import {
  inputString,
  stringInput,
  integerInput,
  headerObject,
  headersObject,
  headerUpdateObject,
  modelWritesArray,
  modelCapabilitiesArray,
  modelLimitsObject,
  templateLimitsObject,
  templatePriceObject,
  bodyDefaultsArray,
  bodyDefaultObject,
  createProviderObject,
  createProvidersObject,
  providerPatchObject,
} from './runtime.ts'
export const AccessibilityPatchInputConstraints = {} as const
export const AccessibilityPatchInputInput = {}
export const AccessibilityPatchInput = z
  .object({
    underlineLinks: z.boolean().optional(),
    reduceMotion: z.boolean().optional(),
    pointerCursors: z.boolean().optional(),
  })
  .strict()
export type AccessibilityPatchInputFormInput = z.input<typeof AccessibilityPatchInput>
export type AccessibilityPatchInputFormOutput = z.output<typeof AccessibilityPatchInput>
export const AnalyticUserProfilePutInputConstraints = {
  key: {
    minLength: 1,
    maxLength: 64,
    messages: { minLength: 'Enter a key of 1 to 64 characters.', maxLength: 'Enter a key of 1 to 64 characters.' },
  },
  value: { maxLength: 256, messages: { maxLength: 'Value must be 256 characters or fewer.' } },
} as const
export const AnalyticUserProfilePutInputInput = {
  key: stringInput(AnalyticUserProfilePutInputConstraints['key']),
  value: stringInput(AnalyticUserProfilePutInputConstraints['value']),
}
export const AnalyticUserProfilePutInput = z
  .object({
    key: inputString({
      minLength: 1,
      maxLength: 64,
      messages: { minLength: 'Enter a key of 1 to 64 characters.', maxLength: 'Enter a key of 1 to 64 characters.' },
    }),
    value: inputString({ maxLength: 256, messages: { maxLength: 'Value must be 256 characters or fewer.' } }),
  })
  .strict()
export type AnalyticUserProfilePutInputFormInput = z.input<typeof AnalyticUserProfilePutInput>
export type AnalyticUserProfilePutInputFormOutput = z.output<typeof AnalyticUserProfilePutInput>
export const AuthenticationPurpose = z.enum(['LOGIN', 'PASSKEY_CHANGE'])
export type AuthenticationPurposeFormInput = z.input<typeof AuthenticationPurpose>
export type AuthenticationPurposeFormOutput = z.output<typeof AuthenticationPurpose>
export const PreferenceWeekday = z.enum(['MONDAY', 'SUNDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY'])
export type PreferenceWeekdayFormInput = z.input<typeof PreferenceWeekday>
export type PreferenceWeekdayFormOutput = z.output<typeof PreferenceWeekday>
export const CalendarPatchInputConstraints = {} as const
export const CalendarPatchInputInput = {}
export const CalendarPatchInput = z.object({ firstDay: PreferenceWeekday.optional() }).strict()
export type CalendarPatchInputFormInput = z.input<typeof CalendarPatchInput>
export type CalendarPatchInputFormOutput = z.output<typeof CalendarPatchInput>
export const ChatCreateInputConstraints = {
  message: {
    minLength: 1,
    messages: { minLength: 'Enter a message.', 'x-compallm-format': 'Enter a message.' },
    format: 'nonblank',
  },
  modelId: { minLength: 1, messages: { minLength: 'Choose a model.' } },
} as const
export const ChatCreateInputInput = {
  message: stringInput(ChatCreateInputConstraints['message']),
  modelId: stringInput(ChatCreateInputConstraints['modelId']),
}
export const ChatCreateInput = z
  .object({
    message: inputString({
      minLength: 1,
      messages: { minLength: 'Enter a message.', 'x-compallm-format': 'Enter a message.' },
      format: 'nonblank',
    }),
    modelId: inputString({ minLength: 1, messages: { minLength: 'Choose a model.' } }),
  })
  .strict()
export type ChatCreateInputFormInput = z.input<typeof ChatCreateInput>
export type ChatCreateInputFormOutput = z.output<typeof ChatCreateInput>
export const PreferenceCodeTheme = z.enum(['FOLLOW_INTERFACE', 'SYSTEM', 'LINEAR_LIGHT', 'LINEAR_DARK'])
export type PreferenceCodeThemeFormInput = z.input<typeof PreferenceCodeTheme>
export type PreferenceCodeThemeFormOutput = z.output<typeof PreferenceCodeTheme>
export const PreferenceFontFamily = z.enum(['DEFAULT', 'MONOSPACE'])
export type PreferenceFontFamilyFormInput = z.input<typeof PreferenceFontFamily>
export type PreferenceFontFamilyFormOutput = z.output<typeof PreferenceFontFamily>
export const PreferencePixelFontSize = z.enum(['PX12', 'PX13', 'PX14', 'PX16'])
export type PreferencePixelFontSizeFormInput = z.input<typeof PreferencePixelFontSize>
export type PreferencePixelFontSizeFormOutput = z.output<typeof PreferencePixelFontSize>
export const CodeFontPatchInputConstraints = {} as const
export const CodeFontPatchInputInput = {}
export const CodeFontPatchInput = z
  .object({ family: PreferenceFontFamily.optional(), size: PreferencePixelFontSize.optional() })
  .strict()
export type CodeFontPatchInputFormInput = z.input<typeof CodeFontPatchInput>
export type CodeFontPatchInputFormOutput = z.output<typeof CodeFontPatchInput>
export const CodeAppearancePatchInputConstraints = {} as const
export const CodeAppearancePatchInputInput = {}
export const CodeAppearancePatchInput = z
  .object({ theme: PreferenceCodeTheme.optional(), font: CodeFontPatchInput.optional() })
  .strict()
export type CodeAppearancePatchInputFormInput = z.input<typeof CodeAppearancePatchInput>
export type CodeAppearancePatchInputFormOutput = z.output<typeof CodeAppearancePatchInput>
/** Positive u32 context size, encoded as an exact JSON number (1–4294967295). */
export const ContextTokens = z
  .int()
  .min(1)
  .max(4294967295)
  .describe('Positive u32 context size, encoded as an exact JSON number (1–4294967295).')
export type ContextTokensFormInput = z.input<typeof ContextTokens>
export type ContextTokensFormOutput = z.output<typeof ContextTokens>
export const CredentialCreateInputConstraints = {
  name: { minLength: 1, messages: { minLength: 'Enter your full name.' }, trim: true },
} as const
export const CredentialCreateInputInput = { name: stringInput(CredentialCreateInputConstraints['name']) }
export const CredentialCreateInput = z
  .object({ name: inputString({ minLength: 1, messages: { minLength: 'Enter your full name.' }, trim: true }) })
  .strict()
export type CredentialCreateInputFormInput = z.input<typeof CredentialCreateInput>
export type CredentialCreateInputFormOutput = z.output<typeof CredentialCreateInput>
/** How the request delivers credentials; this is never stored in signed claims. */
export const CredentialDelivery = z
  .enum(['COOKIE', 'TOKEN'])
  .describe('How the request delivers credentials; this is never stored in signed claims.')
export type CredentialDeliveryFormInput = z.input<typeof CredentialDelivery>
export type CredentialDeliveryFormOutput = z.output<typeof CredentialDelivery>
export const EmailCodeInputConstraints = {
  code: {
    minLength: 6,
    maxLength: 6,
    pattern: '^[0-9]*$',
    messages: {
      minLength: 'Enter the six-digit verification code.',
      maxLength: 'Enter the six-digit verification code.',
      pattern: 'Use digits only for the verification code.',
    },
    trim: true,
  },
} as const
export const EmailCodeInputInput = { code: stringInput(EmailCodeInputConstraints['code']) }
/** Shared input for confirming a signup or signin email challenge.

The verified encrypted challenge comes from request context, never this input.
The adapter calls `validify()` before Core verifies the code and challenge. */
export const EmailCodeInput = z
  .object({
    purpose: AuthenticationPurpose.default('LOGIN'),
    code: inputString({
      minLength: 6,
      maxLength: 6,
      pattern: '^[0-9]*$',
      messages: {
        minLength: 'Enter the six-digit verification code.',
        maxLength: 'Enter the six-digit verification code.',
        pattern: 'Use digits only for the verification code.',
      },
      trim: true,
    }).describe('Exactly six ASCII digits after trimming surrounding whitespace.'),
    delivery: CredentialDelivery.default('COOKIE').describe(
      'Cookie delivery is the default; token delivery must be requested explicitly.',
    ),
  })
  .strict()
  .describe(
    'Shared input for confirming a signup or signin email challenge.\n\nThe verified encrypted challenge comes from request context, never this input.\nThe adapter calls `validify()` before Core verifies the code and challenge.',
  )
export type EmailCodeInputFormInput = z.input<typeof EmailCodeInput>
export type EmailCodeInputFormOutput = z.output<typeof EmailCodeInput>
export const EmailInputConstraints = {
  email: {
    messages: {
      'x-compallm-format': 'Enter a valid email address, such as name@example.com.',
      'x-compallm-email-policy': 'Enter a valid email address, such as name@example.com.',
    },
    format: 'emailAddress',
    trim: true,
    asciiLowercase: true,
    emailPolicy: {
      minimumDomainSegments: 2,
      allowDomainLiteral: false,
      localMaxBytes: 64,
      labelMaxBytes: 63,
      domainMaxBytes: 254,
      atom: '^[\\u{21}\\u{23}-\\u{27}\\u{2a}-\\u{2b}\\u{2d}\\u{2f}-\\u{39}\\u{3d}\\u{3f}\\u{41}-\\u{5a}\\u{5e}-\\u{7e}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
      quoted:
        '^[\\u{9}\\u{20}-\\u{21}\\u{23}-\\u{5b}\\u{5d}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
      literal:
        '^[\\u{21}-\\u{5a}\\u{5e}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
      edge: '^[\\u{30}-\\u{39}\\u{41}-\\u{5a}\\u{61}-\\u{7a}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7a3}\\u{d7b0}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
    },
  },
} as const
export const EmailInputInput = { email: stringInput(EmailInputConstraints['email']) }
/** Shared input for requesting either a signup or a signin email challenge.

Call `validify()` before Core execution. GraphQL parsing alone does not run
normalization, application validation, or email ownership checks. */
export const EmailInput = z
  .object({
    purpose: AuthenticationPurpose.default('LOGIN'),
    email: inputString({
      messages: {
        'x-compallm-format': 'Enter a valid email address, such as name@example.com.',
        'x-compallm-email-policy': 'Enter a valid email address, such as name@example.com.',
      },
      format: 'emailAddress',
      trim: true,
      asciiLowercase: true,
      emailPolicy: {
        minimumDomainSegments: 2,
        allowDomainLiteral: false,
        localMaxBytes: 64,
        labelMaxBytes: 63,
        domainMaxBytes: 254,
        atom: '^[\\u{21}\\u{23}-\\u{27}\\u{2a}-\\u{2b}\\u{2d}\\u{2f}-\\u{39}\\u{3d}\\u{3f}\\u{41}-\\u{5a}\\u{5e}-\\u{7e}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
        quoted:
          '^[\\u{9}\\u{20}-\\u{21}\\u{23}-\\u{5b}\\u{5d}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
        literal:
          '^[\\u{21}-\\u{5a}\\u{5e}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
        edge: '^[\\u{30}-\\u{39}\\u{41}-\\u{5a}\\u{61}-\\u{7a}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7a3}\\u{d7b0}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
      },
    }).describe('Trimmed and ASCII-lowercased using the shared Rust email policy.'),
    invitation: z.boolean().default(false),
    delivery: CredentialDelivery.default('COOKIE').describe(
      'Cookie delivery is the default; token delivery must be requested explicitly.',
    ),
  })
  .strict()
  .describe(
    'Shared input for requesting either a signup or a signin email challenge.\n\nCall `validify()` before Core execution. GraphQL parsing alone does not run\nnormalization, application validation, or email ownership checks.',
  )
export type EmailInputFormInput = z.input<typeof EmailInput>
export type EmailInputFormOutput = z.output<typeof EmailInput>
export const FilterIs = z.enum(['NULL', 'NOT_NULL'])
export type FilterIsFormInput = z.input<typeof FilterIs>
export type FilterIsFormOutput = z.output<typeof FilterIs>
export const GroupCreateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 128,
    messages: {
      minLength: 'Group name must contain 1 to 128 characters.',
      maxLength: 'Group name must contain 1 to 128 characters.',
    },
    trim: true,
  },
  description: { maxLength: 128, messages: { maxLength: 'Description must be 128 characters or fewer.' } },
  roleIds: { maxItems: 32, messages: { maxItems: 'Choose no more than 32 roles.' } },
} as const
export const GroupCreateInputInput = {
  name: stringInput(GroupCreateInputConstraints['name']),
  description: stringInput(GroupCreateInputConstraints['description']),
}
export const GroupCreateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 128,
      messages: {
        minLength: 'Group name must contain 1 to 128 characters.',
        maxLength: 'Group name must contain 1 to 128 characters.',
      },
      trim: true,
    }),
    description: inputString({
      maxLength: 128,
      messages: { maxLength: 'Description must be 128 characters or fewer.' },
    }),
    roleIds: z.array(inputString({})).max(32, { message: 'Choose no more than 32 roles.' }),
  })
  .strict()
export type GroupCreateInputFormInput = z.input<typeof GroupCreateInput>
export type GroupCreateInputFormOutput = z.output<typeof GroupCreateInput>
export const GroupUpdateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 128,
    messages: { minLength: 'Name is required.', maxLength: 'Name must be 128 characters or fewer.' },
    trim: true,
  },
  description: { maxLength: 128, messages: { maxLength: 'Description must be 128 characters or fewer.' } },
  addMemberIds: { maxItems: 100, messages: { maxItems: 'Add no more than 100 members at a time.' } },
  removeMemberIds: { maxItems: 100, messages: { maxItems: 'Remove no more than 100 members at a time.' } },
  addRoleIds: { maxItems: 32, messages: { maxItems: 'Add no more than 32 roles at a time.' } },
  removeRoleIds: { maxItems: 32, messages: { maxItems: 'Remove no more than 32 roles at a time.' } },
} as const
export const GroupUpdateInputInput = {
  name: stringInput(GroupUpdateInputConstraints['name']),
  description: stringInput(GroupUpdateInputConstraints['description']),
}
export const GroupUpdateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 128,
      messages: { minLength: 'Name is required.', maxLength: 'Name must be 128 characters or fewer.' },
      trim: true,
    })
      .nullable()
      .optional(),
    description: inputString({
      maxLength: 128,
      messages: { maxLength: 'Description must be 128 characters or fewer.' },
    })
      .nullable()
      .optional(),
    addMemberIds: z
      .array(inputString({}))
      .max(100, { message: 'Add no more than 100 members at a time.' })
      .nullable()
      .optional(),
    removeMemberIds: z
      .array(inputString({}))
      .max(100, { message: 'Remove no more than 100 members at a time.' })
      .nullable()
      .optional(),
    addRoleIds: z
      .array(inputString({}))
      .max(32, { message: 'Add no more than 32 roles at a time.' })
      .nullable()
      .optional(),
    removeRoleIds: z
      .array(inputString({}))
      .max(32, { message: 'Remove no more than 32 roles at a time.' })
      .nullable()
      .optional(),
  })
  .strict()
export type GroupUpdateInputFormInput = z.input<typeof GroupUpdateInput>
export type GroupUpdateInputFormOutput = z.output<typeof GroupUpdateInput>
export const HomeDestination = z.enum(['CHAT', 'MODELS', 'PROVIDERS'])
export type HomeDestinationFormInput = z.input<typeof HomeDestination>
export type HomeDestinationFormOutput = z.output<typeof HomeDestination>
export const IDFilterConstraints = {} as const
export const IDFilterInput = {}
export const IDFilter = z.object({ eq: inputString({}).nullable().optional() }).strict()
export type IDFilterFormInput = z.input<typeof IDFilter>
export type IDFilterFormOutput = z.output<typeof IDFilter>
export const ImageMediaType = z.enum(['JPEG', 'PNG', 'WEBP'])
export type ImageMediaTypeFormInput = z.input<typeof ImageMediaType>
export type ImageMediaTypeFormOutput = z.output<typeof ImageMediaType>
export const ImageUploadInputConstraints = {
  mediaType: { messages: { enum: 'Choose a JPEG, PNG, or WebP image.' } },
  size: {
    minimum: 1,
    maximum: 5242880,
    messages: {
      minimum: 'Choose a profile photo that is not empty.',
      maximum: 'Profile photos must be 5 MiB or smaller.',
    },
  },
} as const
export const ImageUploadInputInput = {
  mediaType: stringInput(ImageUploadInputConstraints['mediaType']),
  size: integerInput({ minimum: 1, maximum: 5242880 }),
}
export const ImageUploadInput = z
  .object({
    mediaType: z.enum(ImageMediaType.options, { error: 'Choose a JPEG, PNG, or WebP image.' }),
    size: z
      .int()
      .min(1, { message: 'Choose a profile photo that is not empty.' })
      .max(5242880, { message: 'Profile photos must be 5 MiB or smaller.' }),
  })
  .strict()
export type ImageUploadInputFormInput = z.input<typeof ImageUploadInput>
export type ImageUploadInputFormOutput = z.output<typeof ImageUploadInput>
export const InvitationCreateInputConstraints = {
  email: {
    messages: {
      'x-compallm-format': 'Enter a valid email address, such as name@example.com.',
      'x-compallm-email-policy': 'Enter a valid email address, such as name@example.com.',
    },
    format: 'emailAddress',
    trim: true,
    asciiLowercase: true,
    emailPolicy: {
      minimumDomainSegments: 2,
      allowDomainLiteral: false,
      localMaxBytes: 64,
      labelMaxBytes: 63,
      domainMaxBytes: 254,
      atom: '^[\\u{21}\\u{23}-\\u{27}\\u{2a}-\\u{2b}\\u{2d}\\u{2f}-\\u{39}\\u{3d}\\u{3f}\\u{41}-\\u{5a}\\u{5e}-\\u{7e}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
      quoted:
        '^[\\u{9}\\u{20}-\\u{21}\\u{23}-\\u{5b}\\u{5d}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
      literal:
        '^[\\u{21}-\\u{5a}\\u{5e}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
      edge: '^[\\u{30}-\\u{39}\\u{41}-\\u{5a}\\u{61}-\\u{7a}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7a3}\\u{d7b0}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
    },
  },
} as const
export const InvitationCreateInputInput = { email: stringInput(InvitationCreateInputConstraints['email']) }
export const InvitationCreateInput = z
  .object({
    email: inputString({
      messages: {
        'x-compallm-format': 'Enter a valid email address, such as name@example.com.',
        'x-compallm-email-policy': 'Enter a valid email address, such as name@example.com.',
      },
      format: 'emailAddress',
      trim: true,
      asciiLowercase: true,
      emailPolicy: {
        minimumDomainSegments: 2,
        allowDomainLiteral: false,
        localMaxBytes: 64,
        labelMaxBytes: 63,
        domainMaxBytes: 254,
        atom: '^[\\u{21}\\u{23}-\\u{27}\\u{2a}-\\u{2b}\\u{2d}\\u{2f}-\\u{39}\\u{3d}\\u{3f}\\u{41}-\\u{5a}\\u{5e}-\\u{7e}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
        quoted:
          '^[\\u{9}\\u{20}-\\u{21}\\u{23}-\\u{5b}\\u{5d}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
        literal:
          '^[\\u{21}-\\u{5a}\\u{5e}-\\u{7e}\\u{c280}-\\u{c2bf}\\u{c380}-\\u{c3bf}\\u{c480}-\\u{c4bf}\\u{c580}-\\u{c5bf}\\u{c680}-\\u{c6bf}\\u{c780}-\\u{c7bf}\\u{c880}-\\u{c8bf}\\u{c980}-\\u{c9bf}\\u{ca80}-\\u{cabf}\\u{cb80}-\\u{cbbf}\\u{cc80}-\\u{ccbf}\\u{cd80}-\\u{cdbf}\\u{ce80}-\\u{cebf}\\u{cf80}-\\u{cfbf}\\u{d080}-\\u{d0bf}\\u{d180}-\\u{d1bf}\\u{d280}-\\u{d2bf}\\u{d380}-\\u{d3bf}\\u{d480}-\\u{d4bf}\\u{d580}-\\u{d5bf}\\u{d680}-\\u{d6bf}\\u{d780}-\\u{d7bf}]+$',
        edge: '^[\\u{30}-\\u{39}\\u{41}-\\u{5a}\\u{61}-\\u{7a}\\u{aa}\\u{b2}-\\u{b3}\\u{b5}\\u{b9}-\\u{ba}\\u{bc}-\\u{be}\\u{c0}-\\u{d6}\\u{d8}-\\u{f6}\\u{f8}-\\u{2c1}\\u{2c6}-\\u{2d1}\\u{2e0}-\\u{2e4}\\u{2ec}\\u{2ee}\\u{345}\\u{363}-\\u{374}\\u{376}-\\u{377}\\u{37a}-\\u{37d}\\u{37f}\\u{386}\\u{388}-\\u{38a}\\u{38c}\\u{38e}-\\u{3a1}\\u{3a3}-\\u{3f5}\\u{3f7}-\\u{481}\\u{48a}-\\u{52f}\\u{531}-\\u{556}\\u{558}-\\u{559}\\u{560}-\\u{588}\\u{58b}-\\u{58c}\\u{5b0}-\\u{5bd}\\u{5bf}\\u{5c1}-\\u{5c2}\\u{5c4}-\\u{5c5}\\u{5c7}-\\u{5c9}\\u{5d0}-\\u{5ea}\\u{5ef}-\\u{5f2}\\u{610}-\\u{61a}\\u{620}-\\u{657}\\u{659}-\\u{669}\\u{66e}-\\u{6d3}\\u{6d5}-\\u{6dc}\\u{6e1}-\\u{6e8}\\u{6ed}-\\u{6fc}\\u{6ff}\\u{710}-\\u{73f}\\u{74d}-\\u{7b1}\\u{7c0}-\\u{7ea}\\u{7f4}-\\u{7f5}\\u{7fa}\\u{800}-\\u{817}\\u{81a}-\\u{82c}\\u{840}-\\u{858}\\u{860}-\\u{86a}\\u{870}-\\u{887}\\u{889}-\\u{88f}\\u{897}\\u{8a0}-\\u{8c9}\\u{8d4}-\\u{8df}\\u{8e3}-\\u{8e9}\\u{8f0}-\\u{93b}\\u{93d}-\\u{94c}\\u{94e}-\\u{950}\\u{955}-\\u{963}\\u{966}-\\u{96f}\\u{971}-\\u{983}\\u{985}-\\u{98c}\\u{98f}-\\u{990}\\u{993}-\\u{9a8}\\u{9aa}-\\u{9b0}\\u{9b2}\\u{9b6}-\\u{9b9}\\u{9bd}-\\u{9c4}\\u{9c7}-\\u{9c8}\\u{9cb}-\\u{9cc}\\u{9ce}\\u{9d7}\\u{9dc}-\\u{9dd}\\u{9df}-\\u{9e3}\\u{9e6}-\\u{9f1}\\u{9f4}-\\u{9f9}\\u{9fc}\\u{a01}-\\u{a03}\\u{a05}-\\u{a0a}\\u{a0f}-\\u{a10}\\u{a13}-\\u{a28}\\u{a2a}-\\u{a30}\\u{a32}-\\u{a33}\\u{a35}-\\u{a36}\\u{a38}-\\u{a39}\\u{a3e}-\\u{a42}\\u{a47}-\\u{a48}\\u{a4b}-\\u{a4c}\\u{a51}\\u{a59}-\\u{a5c}\\u{a5e}\\u{a66}-\\u{a75}\\u{a81}-\\u{a83}\\u{a85}-\\u{a8d}\\u{a8f}-\\u{a91}\\u{a93}-\\u{aa8}\\u{aaa}-\\u{ab0}\\u{ab2}-\\u{ab3}\\u{ab5}-\\u{ab9}\\u{abd}-\\u{ac5}\\u{ac7}-\\u{ac9}\\u{acb}-\\u{acc}\\u{ad0}\\u{ae0}-\\u{ae3}\\u{ae6}-\\u{aef}\\u{af9}-\\u{afc}\\u{b01}-\\u{b03}\\u{b05}-\\u{b0c}\\u{b0f}-\\u{b10}\\u{b13}-\\u{b28}\\u{b2a}-\\u{b30}\\u{b32}-\\u{b33}\\u{b35}-\\u{b39}\\u{b3d}-\\u{b44}\\u{b47}-\\u{b48}\\u{b4b}-\\u{b4c}\\u{b56}-\\u{b57}\\u{b5c}-\\u{b5d}\\u{b5f}-\\u{b63}\\u{b66}-\\u{b6f}\\u{b71}-\\u{b77}\\u{b82}-\\u{b83}\\u{b85}-\\u{b8a}\\u{b8e}-\\u{b90}\\u{b92}-\\u{b95}\\u{b99}-\\u{b9a}\\u{b9c}\\u{b9e}-\\u{b9f}\\u{ba3}-\\u{ba4}\\u{ba8}-\\u{baa}\\u{bae}-\\u{bb9}\\u{bbe}-\\u{bc2}\\u{bc6}-\\u{bc8}\\u{bca}-\\u{bcc}\\u{bd0}\\u{bd7}\\u{be6}-\\u{bf2}\\u{c00}-\\u{c0c}\\u{c0e}-\\u{c10}\\u{c12}-\\u{c28}\\u{c2a}-\\u{c39}\\u{c3d}-\\u{c44}\\u{c46}-\\u{c48}\\u{c4a}-\\u{c4c}\\u{c55}-\\u{c56}\\u{c58}-\\u{c5a}\\u{c5c}-\\u{c5d}\\u{c60}-\\u{c63}\\u{c66}-\\u{c6f}\\u{c78}-\\u{c7e}\\u{c80}-\\u{c83}\\u{c85}-\\u{c8c}\\u{c8e}-\\u{c90}\\u{c92}-\\u{ca8}\\u{caa}-\\u{cb3}\\u{cb5}-\\u{cb9}\\u{cbd}-\\u{cc4}\\u{cc6}-\\u{cc8}\\u{cca}-\\u{ccc}\\u{cd5}-\\u{cd6}\\u{cdc}-\\u{cde}\\u{ce0}-\\u{ce3}\\u{ce6}-\\u{cef}\\u{cf1}-\\u{cf3}\\u{d00}-\\u{d0c}\\u{d0e}-\\u{d10}\\u{d12}-\\u{d3a}\\u{d3d}-\\u{d44}\\u{d46}-\\u{d48}\\u{d4a}-\\u{d4c}\\u{d4e}\\u{d54}-\\u{d63}\\u{d66}-\\u{d78}\\u{d7a}-\\u{d7f}\\u{d81}-\\u{d83}\\u{d85}-\\u{d96}\\u{d9a}-\\u{db1}\\u{db3}-\\u{dbb}\\u{dbd}\\u{dc0}-\\u{dc6}\\u{dcf}-\\u{dd4}\\u{dd6}\\u{dd8}-\\u{ddf}\\u{de6}-\\u{def}\\u{df2}-\\u{df3}\\u{e01}-\\u{e3a}\\u{e40}-\\u{e46}\\u{e4d}\\u{e50}-\\u{e59}\\u{e81}-\\u{e82}\\u{e84}\\u{e86}-\\u{e8a}\\u{e8c}-\\u{ea3}\\u{ea5}\\u{ea7}-\\u{eb9}\\u{ebb}-\\u{ebd}\\u{ec0}-\\u{ec4}\\u{ec6}\\u{ecd}\\u{ed0}-\\u{ed9}\\u{edc}-\\u{edf}\\u{f00}\\u{f20}-\\u{f33}\\u{f40}-\\u{f47}\\u{f49}-\\u{f6c}\\u{f71}-\\u{f83}\\u{f88}-\\u{f97}\\u{f99}-\\u{fbc}\\u{1000}-\\u{1036}\\u{1038}\\u{103b}-\\u{1049}\\u{1050}-\\u{109d}\\u{10a0}-\\u{10c5}\\u{10c7}\\u{10cd}\\u{10d0}-\\u{10fa}\\u{10fc}-\\u{1248}\\u{124a}-\\u{124d}\\u{1250}-\\u{1256}\\u{1258}\\u{125a}-\\u{125d}\\u{1260}-\\u{1288}\\u{128a}-\\u{128d}\\u{1290}-\\u{12b0}\\u{12b2}-\\u{12b5}\\u{12b8}-\\u{12be}\\u{12c0}\\u{12c2}-\\u{12c5}\\u{12c8}-\\u{12d6}\\u{12d8}-\\u{1310}\\u{1312}-\\u{1315}\\u{1318}-\\u{135a}\\u{1369}-\\u{137c}\\u{1380}-\\u{138f}\\u{13a0}-\\u{13f5}\\u{13f8}-\\u{13fd}\\u{1401}-\\u{166c}\\u{166f}-\\u{167f}\\u{1681}-\\u{169a}\\u{16a0}-\\u{16ea}\\u{16ee}-\\u{16f8}\\u{1700}-\\u{1713}\\u{171f}-\\u{1733}\\u{1740}-\\u{1753}\\u{1760}-\\u{176c}\\u{176e}-\\u{1770}\\u{1772}-\\u{1773}\\u{1780}-\\u{17b3}\\u{17b6}-\\u{17c8}\\u{17d7}\\u{17dc}\\u{17e0}-\\u{17e9}\\u{17f0}-\\u{17f9}\\u{1810}-\\u{1819}\\u{1820}-\\u{1878}\\u{1880}-\\u{18aa}\\u{18b0}-\\u{18f5}\\u{1900}-\\u{191e}\\u{1920}-\\u{192b}\\u{1930}-\\u{1938}\\u{1946}-\\u{196d}\\u{1970}-\\u{1974}\\u{1980}-\\u{19ab}\\u{19b0}-\\u{19c9}\\u{19d0}-\\u{19da}\\u{1a00}-\\u{1a1b}\\u{1a20}-\\u{1a5e}\\u{1a61}-\\u{1a74}\\u{1a80}-\\u{1a89}\\u{1a90}-\\u{1a99}\\u{1aa7}\\u{1abf}-\\u{1ac0}\\u{1acc}-\\u{1ace}\\u{1b00}-\\u{1b33}\\u{1b35}-\\u{1b43}\\u{1b45}-\\u{1b4c}\\u{1b50}-\\u{1b59}\\u{1b80}-\\u{1ba9}\\u{1bac}-\\u{1be5}\\u{1be7}-\\u{1bf1}\\u{1c00}-\\u{1c36}\\u{1c40}-\\u{1c49}\\u{1c4d}-\\u{1c7d}\\u{1c80}-\\u{1c8a}\\u{1c90}-\\u{1cba}\\u{1cbd}-\\u{1cbf}\\u{1ce9}-\\u{1cec}\\u{1cee}-\\u{1cf3}\\u{1cf5}-\\u{1cf6}\\u{1cfa}\\u{1d00}-\\u{1dbf}\\u{1dd3}-\\u{1df4}\\u{1e00}-\\u{1f15}\\u{1f18}-\\u{1f1d}\\u{1f20}-\\u{1f45}\\u{1f48}-\\u{1f4d}\\u{1f50}-\\u{1f57}\\u{1f59}\\u{1f5b}\\u{1f5d}\\u{1f5f}-\\u{1f7d}\\u{1f80}-\\u{1fb4}\\u{1fb6}-\\u{1fbc}\\u{1fbe}\\u{1fc2}-\\u{1fc4}\\u{1fc6}-\\u{1fcc}\\u{1fd0}-\\u{1fd3}\\u{1fd6}-\\u{1fdb}\\u{1fe0}-\\u{1fec}\\u{1ff2}-\\u{1ff4}\\u{1ff6}-\\u{1ffc}\\u{2070}-\\u{2071}\\u{2074}-\\u{2079}\\u{207f}-\\u{2089}\\u{208f}-\\u{209f}\\u{2102}\\u{2107}\\u{210a}-\\u{2113}\\u{2115}\\u{2119}-\\u{211d}\\u{2124}\\u{2126}\\u{2128}\\u{212a}-\\u{212d}\\u{212f}-\\u{2139}\\u{213c}-\\u{213f}\\u{2145}-\\u{2149}\\u{214e}\\u{2150}-\\u{2189}\\u{2460}-\\u{249b}\\u{24b6}-\\u{24ff}\\u{2776}-\\u{2793}\\u{2c00}-\\u{2ce4}\\u{2ceb}-\\u{2cee}\\u{2cf2}-\\u{2cf3}\\u{2cfd}\\u{2d00}-\\u{2d25}\\u{2d27}\\u{2d2d}\\u{2d30}-\\u{2d67}\\u{2d6f}\\u{2d80}-\\u{2d96}\\u{2da0}-\\u{2da6}\\u{2da8}-\\u{2dae}\\u{2db0}-\\u{2db6}\\u{2db8}-\\u{2dbe}\\u{2dc0}-\\u{2dc6}\\u{2dc8}-\\u{2dce}\\u{2dd0}-\\u{2dd6}\\u{2dd8}-\\u{2dde}\\u{2de0}-\\u{2dff}\\u{2e2f}\\u{3005}-\\u{3007}\\u{3021}-\\u{3029}\\u{3031}-\\u{3035}\\u{3038}-\\u{303c}\\u{3041}-\\u{3096}\\u{309d}-\\u{309f}\\u{30a1}-\\u{30fa}\\u{30fc}-\\u{30ff}\\u{3105}-\\u{312f}\\u{3131}-\\u{318e}\\u{3192}-\\u{3195}\\u{31a0}-\\u{31bf}\\u{31f0}-\\u{31ff}\\u{3220}-\\u{3229}\\u{3248}-\\u{324f}\\u{3251}-\\u{325f}\\u{3280}-\\u{3289}\\u{32b1}-\\u{32bf}\\u{3400}-\\u{4dbf}\\u{4e00}-\\u{a48c}\\u{a4d0}-\\u{a4fd}\\u{a500}-\\u{a60c}\\u{a610}-\\u{a62b}\\u{a640}-\\u{a66e}\\u{a674}-\\u{a67b}\\u{a67f}-\\u{a6ef}\\u{a717}-\\u{a71f}\\u{a722}-\\u{a788}\\u{a78b}-\\u{a7dd}\\u{a7e2}\\u{a7f1}-\\u{a805}\\u{a807}-\\u{a827}\\u{a830}-\\u{a835}\\u{a840}-\\u{a873}\\u{a880}-\\u{a8c3}\\u{a8c5}\\u{a8d0}-\\u{a8d9}\\u{a8f2}-\\u{a8f7}\\u{a8fb}\\u{a8fd}-\\u{a92a}\\u{a930}-\\u{a952}\\u{a960}-\\u{a97c}\\u{a980}-\\u{a9b2}\\u{a9b4}-\\u{a9bf}\\u{a9cf}-\\u{a9d9}\\u{a9e0}-\\u{a9fe}\\u{aa00}-\\u{aa36}\\u{aa40}-\\u{aa4d}\\u{aa50}-\\u{aa59}\\u{aa60}-\\u{aa76}\\u{aa7a}-\\u{aabe}\\u{aac0}\\u{aac2}\\u{aadb}-\\u{aadd}\\u{aae0}-\\u{aaef}\\u{aaf2}-\\u{aaf5}\\u{ab01}-\\u{ab06}\\u{ab09}-\\u{ab0e}\\u{ab11}-\\u{ab16}\\u{ab20}-\\u{ab26}\\u{ab28}-\\u{ab2e}\\u{ab30}-\\u{ab5a}\\u{ab5c}-\\u{ab69}\\u{ab6c}-\\u{ab6d}\\u{ab70}-\\u{abea}\\u{abf0}-\\u{abf9}\\u{ac00}-\\u{d7a3}\\u{d7b0}-\\u{d7c6}\\u{d7cb}-\\u{d7fb}\\u{f900}-\\u{fa6d}\\u{fa70}-\\u{fad9}\\u{fb00}-\\u{fb06}\\u{fb13}-\\u{fb17}\\u{fb1d}-\\u{fb28}\\u{fb2a}-\\u{fb36}\\u{fb38}-\\u{fb3c}\\u{fb3e}\\u{fb40}-\\u{fb41}\\u{fb43}-\\u{fb44}\\u{fb46}-\\u{fbb1}\\u{fbd3}-\\u{fd3d}\\u{fd50}-\\u{fd8f}\\u{fd92}-\\u{fdc7}\\u{fdf0}-\\u{fdfb}\\u{fe70}-\\u{fe74}\\u{fe76}-\\u{fefc}\\u{ff10}-\\u{ff19}\\u{ff21}-\\u{ff3a}\\u{ff41}-\\u{ff5a}\\u{ff66}-\\u{ffbe}\\u{ffc2}-\\u{ffc7}\\u{ffca}-\\u{ffcf}\\u{ffd2}-\\u{ffd7}\\u{ffda}-\\u{ffdc}\\u{10000}-\\u{1000b}\\u{1000d}-\\u{10026}\\u{10028}-\\u{1003a}\\u{1003c}-\\u{1003d}\\u{1003f}-\\u{1004d}\\u{10050}-\\u{1005d}\\u{10080}-\\u{100fa}\\u{10107}-\\u{10133}\\u{10140}-\\u{10178}\\u{1018a}-\\u{1018b}\\u{10280}-\\u{1029c}\\u{102a0}-\\u{102d0}\\u{102e1}-\\u{102fb}\\u{10300}-\\u{10323}\\u{1032d}-\\u{1034a}\\u{10350}-\\u{1037a}\\u{10380}-\\u{1039d}\\u{103a0}-\\u{103c3}\\u{103c8}-\\u{103cf}\\u{103d1}-\\u{103d5}\\u{10400}-\\u{1049d}\\u{104a0}-\\u{104a9}\\u{104b0}-\\u{104d3}\\u{104d8}-\\u{104fb}\\u{10500}-\\u{10527}\\u{10530}-\\u{10563}\\u{10570}-\\u{1057a}\\u{1057c}-\\u{1058a}\\u{1058c}-\\u{10592}\\u{10594}-\\u{10595}\\u{10597}-\\u{105a1}\\u{105a3}-\\u{105b1}\\u{105b3}-\\u{105b9}\\u{105bb}-\\u{105bc}\\u{105c0}-\\u{105f3}\\u{10600}-\\u{10736}\\u{10740}-\\u{10755}\\u{10760}-\\u{10767}\\u{10780}-\\u{10785}\\u{10787}-\\u{107b0}\\u{107b2}-\\u{107bf}\\u{10800}-\\u{10805}\\u{10808}\\u{1080a}-\\u{10835}\\u{10837}-\\u{10838}\\u{1083c}\\u{1083f}-\\u{10855}\\u{10858}-\\u{10876}\\u{10879}-\\u{1089e}\\u{108a7}-\\u{108af}\\u{108e0}-\\u{108f2}\\u{108f4}-\\u{108f5}\\u{108fb}-\\u{1091b}\\u{10920}-\\u{10939}\\u{10940}-\\u{10959}\\u{10980}-\\u{109b7}\\u{109bc}-\\u{109cf}\\u{109d2}-\\u{10a03}\\u{10a05}-\\u{10a06}\\u{10a0c}-\\u{10a13}\\u{10a15}-\\u{10a17}\\u{10a19}-\\u{10a35}\\u{10a40}-\\u{10a48}\\u{10a60}-\\u{10a7e}\\u{10a80}-\\u{10a9f}\\u{10ac0}-\\u{10ac7}\\u{10ac9}-\\u{10ae4}\\u{10aeb}-\\u{10aef}\\u{10b00}-\\u{10b35}\\u{10b40}-\\u{10b55}\\u{10b58}-\\u{10b72}\\u{10b78}-\\u{10b91}\\u{10ba9}-\\u{10baf}\\u{10c00}-\\u{10c48}\\u{10c80}-\\u{10cb2}\\u{10cc0}-\\u{10cf2}\\u{10cfa}-\\u{10d27}\\u{10d30}-\\u{10d39}\\u{10d40}-\\u{10d65}\\u{10d69}\\u{10d6f}-\\u{10d85}\\u{10e60}-\\u{10e7e}\\u{10e80}-\\u{10ea9}\\u{10eab}-\\u{10eac}\\u{10eb0}-\\u{10eb1}\\u{10ec2}-\\u{10ec7}\\u{10ecb}-\\u{10ecd}\\u{10ed9}-\\u{10eee}\\u{10ef3}\\u{10ef5}\\u{10ef7}-\\u{10ef8}\\u{10efa}-\\u{10efc}\\u{10f00}-\\u{10f27}\\u{10f30}-\\u{10f45}\\u{10f51}-\\u{10f54}\\u{10f70}-\\u{10f81}\\u{10fb0}-\\u{10fcb}\\u{10fe0}-\\u{10ff6}\\u{11000}-\\u{11045}\\u{11052}-\\u{1106f}\\u{11071}-\\u{11075}\\u{11080}-\\u{110b8}\\u{110c2}\\u{110d0}-\\u{110e8}\\u{110f0}-\\u{110f9}\\u{11100}-\\u{11132}\\u{11136}-\\u{1113f}\\u{11144}-\\u{11147}\\u{11150}-\\u{11172}\\u{11176}\\u{11180}-\\u{111bf}\\u{111c1}-\\u{111c4}\\u{111ce}-\\u{111da}\\u{111dc}\\u{111e1}-\\u{111f4}\\u{11200}-\\u{11211}\\u{11213}-\\u{11234}\\u{11237}\\u{1123e}-\\u{11241}\\u{11280}-\\u{11286}\\u{11288}\\u{1128a}-\\u{1128d}\\u{1128f}-\\u{1129d}\\u{1129f}-\\u{112a8}\\u{112b0}-\\u{112e8}\\u{112f0}-\\u{112f9}\\u{11300}-\\u{11303}\\u{11305}-\\u{1130c}\\u{1130f}-\\u{11310}\\u{11313}-\\u{11328}\\u{1132a}-\\u{11330}\\u{11332}-\\u{11333}\\u{11335}-\\u{11339}\\u{1133d}-\\u{11344}\\u{11347}-\\u{11348}\\u{1134b}-\\u{1134c}\\u{11350}\\u{11357}\\u{1135d}-\\u{11363}\\u{11380}-\\u{11389}\\u{1138b}\\u{1138e}\\u{11390}-\\u{113b5}\\u{113b7}-\\u{113c0}\\u{113c2}\\u{113c5}\\u{113c7}-\\u{113ca}\\u{113cc}-\\u{113cd}\\u{113d1}\\u{113d3}\\u{11400}-\\u{11441}\\u{11443}-\\u{11445}\\u{11447}-\\u{1144a}\\u{11450}-\\u{11459}\\u{1145f}-\\u{11461}\\u{11480}-\\u{114c1}\\u{114c4}-\\u{114c5}\\u{114c7}\\u{114d0}-\\u{114d9}\\u{11580}-\\u{115b5}\\u{115b8}-\\u{115be}\\u{115d8}-\\u{115dd}\\u{11600}-\\u{1163e}\\u{11640}\\u{11644}\\u{11650}-\\u{11659}\\u{11680}-\\u{116b5}\\u{116b8}\\u{116c0}-\\u{116c9}\\u{116d0}-\\u{116e3}\\u{11700}-\\u{1171a}\\u{1171d}-\\u{1172a}\\u{11730}-\\u{1173b}\\u{11740}-\\u{11746}\\u{11800}-\\u{11838}\\u{118a0}-\\u{118f2}\\u{118ff}-\\u{11906}\\u{11909}\\u{1190c}-\\u{11913}\\u{11915}-\\u{11916}\\u{11918}-\\u{11935}\\u{11937}-\\u{11938}\\u{1193b}-\\u{1193c}\\u{1193f}-\\u{11942}\\u{11950}-\\u{11959}\\u{119a0}-\\u{119a7}\\u{119aa}-\\u{119d7}\\u{119da}-\\u{119df}\\u{119e1}\\u{119e3}-\\u{119e4}\\u{11a00}-\\u{11a32}\\u{11a35}-\\u{11a3e}\\u{11a50}-\\u{11a97}\\u{11a9d}\\u{11ab0}-\\u{11af8}\\u{11b0a}\\u{11b60}-\\u{11b67}\\u{11bc0}-\\u{11be0}\\u{11bf0}-\\u{11bf9}\\u{11c00}-\\u{11c08}\\u{11c0a}-\\u{11c36}\\u{11c38}-\\u{11c3e}\\u{11c40}\\u{11c50}-\\u{11c6c}\\u{11c72}-\\u{11c8f}\\u{11c92}-\\u{11ca7}\\u{11ca9}-\\u{11cb6}\\u{11d00}-\\u{11d06}\\u{11d08}-\\u{11d09}\\u{11d0b}-\\u{11d36}\\u{11d3a}\\u{11d3c}-\\u{11d3d}\\u{11d3f}-\\u{11d41}\\u{11d43}\\u{11d46}-\\u{11d47}\\u{11d50}-\\u{11d59}\\u{11d60}-\\u{11d65}\\u{11d67}-\\u{11d68}\\u{11d6a}-\\u{11d8e}\\u{11d90}-\\u{11d91}\\u{11d93}-\\u{11d96}\\u{11d98}\\u{11da0}-\\u{11da9}\\u{11db0}-\\u{11ddb}\\u{11de0}-\\u{11de9}\\u{11df0}-\\u{11df1}\\u{11ee0}-\\u{11ef6}\\u{11f00}-\\u{11f10}\\u{11f12}-\\u{11f3a}\\u{11f3e}-\\u{11f40}\\u{11f50}-\\u{11f59}\\u{11fb0}\\u{11fc0}-\\u{11fd4}\\u{12000}-\\u{12399}\\u{12400}-\\u{1246f}\\u{12475}-\\u{12543}\\u{12550}-\\u{12686}\\u{12f90}-\\u{12ff0}\\u{13000}-\\u{1342f}\\u{13441}-\\u{13446}\\u{13460}-\\u{143fa}\\u{14400}-\\u{14646}\\u{16100}-\\u{1612e}\\u{16130}-\\u{16139}\\u{16800}-\\u{16a38}\\u{16a40}-\\u{16a5e}\\u{16a60}-\\u{16a69}\\u{16a70}-\\u{16abe}\\u{16ac0}-\\u{16ac9}\\u{16ad0}-\\u{16aed}\\u{16b00}-\\u{16b2f}\\u{16b40}-\\u{16b43}\\u{16b50}-\\u{16b59}\\u{16b5b}-\\u{16b61}\\u{16b63}-\\u{16b77}\\u{16b7d}-\\u{16b8f}\\u{16d40}-\\u{16d6c}\\u{16d70}-\\u{16d79}\\u{16e40}-\\u{16e96}\\u{16ea0}-\\u{16eb8}\\u{16ebb}-\\u{16ed3}\\u{16f00}-\\u{16f4a}\\u{16f4f}-\\u{16f87}\\u{16f8f}-\\u{16f9f}\\u{16fe0}-\\u{16fe1}\\u{16fe3}\\u{16ff0}-\\u{16ff6}\\u{17000}-\\u{18cda}\\u{18cff}-\\u{18d20}\\u{18d80}-\\u{18df2}\\u{18e00}-\\u{19191}\\u{191a0}-\\u{191d2}\\u{1aff0}-\\u{1aff3}\\u{1aff5}-\\u{1affb}\\u{1affd}-\\u{1affe}\\u{1b000}-\\u{1b128}\\u{1b132}\\u{1b150}-\\u{1b152}\\u{1b155}\\u{1b164}-\\u{1b168}\\u{1b170}-\\u{1b2fb}\\u{1bc00}-\\u{1bc6a}\\u{1bc70}-\\u{1bc7c}\\u{1bc80}-\\u{1bc88}\\u{1bc90}-\\u{1bc99}\\u{1bc9e}\\u{1ccf0}-\\u{1ccf9}\\u{1d2c0}-\\u{1d2d3}\\u{1d2e0}-\\u{1d2f3}\\u{1d360}-\\u{1d378}\\u{1d400}-\\u{1d454}\\u{1d456}-\\u{1d49c}\\u{1d49e}-\\u{1d49f}\\u{1d4a2}\\u{1d4a5}-\\u{1d4a6}\\u{1d4a9}-\\u{1d4ac}\\u{1d4ae}-\\u{1d4b9}\\u{1d4bb}\\u{1d4bd}-\\u{1d4c3}\\u{1d4c5}-\\u{1d505}\\u{1d507}-\\u{1d50a}\\u{1d50d}-\\u{1d514}\\u{1d516}-\\u{1d51c}\\u{1d51e}-\\u{1d539}\\u{1d53b}-\\u{1d53e}\\u{1d540}-\\u{1d544}\\u{1d546}\\u{1d54a}-\\u{1d550}\\u{1d552}-\\u{1d6a6}\\u{1d6a8}-\\u{1d6c0}\\u{1d6c2}-\\u{1d6da}\\u{1d6dc}-\\u{1d6fa}\\u{1d6fc}-\\u{1d714}\\u{1d716}-\\u{1d734}\\u{1d736}-\\u{1d74e}\\u{1d750}-\\u{1d76e}\\u{1d770}-\\u{1d788}\\u{1d78a}-\\u{1d7a8}\\u{1d7aa}-\\u{1d7c2}\\u{1d7c4}-\\u{1d7cb}\\u{1d7ce}-\\u{1d7ff}\\u{1df00}-\\u{1df81}\\u{1df90}-\\u{1df96}\\u{1dfcd}-\\u{1e006}\\u{1e008}-\\u{1e018}\\u{1e01b}-\\u{1e021}\\u{1e023}-\\u{1e024}\\u{1e026}-\\u{1e02a}\\u{1e030}-\\u{1e06d}\\u{1e08f}\\u{1e100}-\\u{1e12c}\\u{1e137}-\\u{1e13d}\\u{1e140}-\\u{1e149}\\u{1e14e}\\u{1e290}-\\u{1e2ad}\\u{1e2c0}-\\u{1e2eb}\\u{1e2f0}-\\u{1e2f9}\\u{1e4d0}-\\u{1e4eb}\\u{1e4f0}-\\u{1e4f9}\\u{1e5d0}-\\u{1e5ed}\\u{1e5f0}-\\u{1e5fa}\\u{1e6c0}-\\u{1e6de}\\u{1e6e0}-\\u{1e6f5}\\u{1e6fe}-\\u{1e6ff}\\u{1e7e0}-\\u{1e7e6}\\u{1e7e8}-\\u{1e7eb}\\u{1e7ed}-\\u{1e7ee}\\u{1e7f0}-\\u{1e7fe}\\u{1e800}-\\u{1e8c4}\\u{1e8c7}-\\u{1e8cf}\\u{1e900}-\\u{1e943}\\u{1e947}\\u{1e94b}\\u{1e950}-\\u{1e959}\\u{1ec71}-\\u{1ecab}\\u{1ecad}-\\u{1ecaf}\\u{1ecb1}-\\u{1ecb4}\\u{1ed01}-\\u{1ed2d}\\u{1ed2f}-\\u{1ed3d}\\u{1ee00}-\\u{1ee03}\\u{1ee05}-\\u{1ee1f}\\u{1ee21}-\\u{1ee22}\\u{1ee24}\\u{1ee27}\\u{1ee29}-\\u{1ee32}\\u{1ee34}-\\u{1ee37}\\u{1ee39}\\u{1ee3b}\\u{1ee42}\\u{1ee47}\\u{1ee49}\\u{1ee4b}\\u{1ee4d}-\\u{1ee4f}\\u{1ee51}-\\u{1ee52}\\u{1ee54}\\u{1ee57}\\u{1ee59}\\u{1ee5b}\\u{1ee5d}\\u{1ee5f}\\u{1ee61}-\\u{1ee62}\\u{1ee64}\\u{1ee67}-\\u{1ee6a}\\u{1ee6c}-\\u{1ee72}\\u{1ee74}-\\u{1ee77}\\u{1ee79}-\\u{1ee7c}\\u{1ee7e}\\u{1ee80}-\\u{1ee89}\\u{1ee8b}-\\u{1ee9b}\\u{1eea1}-\\u{1eea3}\\u{1eea5}-\\u{1eea9}\\u{1eeab}-\\u{1eebb}\\u{1f100}-\\u{1f10c}\\u{1f130}-\\u{1f149}\\u{1f150}-\\u{1f169}\\u{1f170}-\\u{1f189}\\u{1fbf0}-\\u{1fbf9}\\u{20000}-\\u{2a6df}\\u{2a700}-\\u{2b81e}\\u{2b820}-\\u{2cead}\\u{2ceb0}-\\u{2ebe0}\\u{2ebf0}-\\u{2ee5d}\\u{2f800}-\\u{2fa1d}\\u{30000}-\\u{3134a}\\u{31350}-\\u{33479}\\u{3d000}-\\u{3fc3f}]+$',
      },
    }),
  })
  .strict()
export type InvitationCreateInputFormInput = z.input<typeof InvitationCreateInput>
export type InvitationCreateInputFormOutput = z.output<typeof InvitationCreateInput>
export const InvitationBatchCreateInputConstraints = {
  invitations: {
    minItems: 1,
    maxItems: 100,
    messages: {
      minItems: 'Invite between 1 and 100 people at a time.',
      maxItems: 'Invite between 1 and 100 people at a time.',
    },
  },
} as const
export const InvitationBatchCreateInputInput = {}
export const InvitationBatchCreateInput = z
  .object({
    invitations: z
      .array(InvitationCreateInput)
      .min(1, { message: 'Invite between 1 and 100 people at a time.' })
      .max(100, { message: 'Invite between 1 and 100 people at a time.' }),
  })
  .strict()
export type InvitationBatchCreateInputFormInput = z.input<typeof InvitationBatchCreateInput>
export type InvitationBatchCreateInputFormOutput = z.output<typeof InvitationBatchCreateInput>
export const PreferenceNamingFormat = z.enum(['FULL_NAME', 'USERNAME'])
export type PreferenceNamingFormatFormInput = z.input<typeof PreferenceNamingFormat>
export type PreferenceNamingFormatFormOutput = z.output<typeof PreferenceNamingFormat>
export const MemberDisplayPatchInputConstraints = {} as const
export const MemberDisplayPatchInputInput = {}
export const MemberDisplayPatchInput = z.object({ nameFormat: PreferenceNamingFormat.optional() }).strict()
export type MemberDisplayPatchInputFormInput = z.input<typeof MemberDisplayPatchInput>
export type MemberDisplayPatchInputFormOutput = z.output<typeof MemberDisplayPatchInput>
export const StringFilterConstraints = {} as const
export const StringFilterInput = {}
export const StringFilter = z
  .object({
    eq: inputString({}).nullable().optional(),
    gt: inputString({}).nullable().optional(),
    gte: inputString({}).nullable().optional(),
    in: z.array(inputString({})).nullable().optional(),
    lt: inputString({}).nullable().optional(),
    lte: inputString({}).nullable().optional(),
    neq: inputString({}).nullable().optional(),
    is: FilterIs.nullable().optional(),
    startsWith: inputString({}).nullable().optional(),
    like: inputString({}).nullable().optional(),
    ilike: inputString({}).nullable().optional(),
    regex: inputString({}).nullable().optional(),
    iregex: inputString({}).nullable().optional(),
  })
  .strict()
export type StringFilterFormInput = z.input<typeof StringFilter>
export type StringFilterFormOutput = z.output<typeof StringFilter>
export const MemberMatchInputConstraints = {} as const
export const MemberMatchInputInput = {}
export const MemberMatchInput = z
  .object({ displayName: StringFilter.nullable().optional(), email: StringFilter.nullable().optional() })
  .strict()
export type MemberMatchInputFormInput = z.input<typeof MemberMatchInput>
export type MemberMatchInputFormOutput = z.output<typeof MemberMatchInput>
export const MemberFilterInputConstraints = {} as const
export const MemberFilterInputInput = {}
export const MemberFilterInput = z
  .object({
    displayName: StringFilter.nullable().optional(),
    email: StringFilter.nullable().optional(),
    or: z.array(MemberMatchInput).nullable().optional(),
    roleId: IDFilter.nullable().optional(),
  })
  .strict()
export type MemberFilterInputFormInput = z.input<typeof MemberFilterInput>
export type MemberFilterInputFormOutput = z.output<typeof MemberFilterInput>
export const OrderByDirection = z.enum(['AscNullsFirst', 'AscNullsLast', 'DescNullsFirst', 'DescNullsLast'])
export type OrderByDirectionFormInput = z.input<typeof OrderByDirection>
export type OrderByDirectionFormOutput = z.output<typeof OrderByDirection>
export const MemberOrderInputConstraints = {} as const
export const MemberOrderInputInput = {}
export const MemberOrderInput = z
  .object({
    id: OrderByDirection.nullable().optional(),
    displayName: OrderByDirection.nullable().optional(),
    email: OrderByDirection.nullable().optional(),
  })
  .strict()
export type MemberOrderInputFormInput = z.input<typeof MemberOrderInput>
export type MemberOrderInputFormOutput = z.output<typeof MemberOrderInput>
export const PreferenceKeyCombination = z.enum(['ENTER', 'MODIFIER_ENTER'])
export type PreferenceKeyCombinationFormInput = z.input<typeof PreferenceKeyCombination>
export type PreferenceKeyCombinationFormOutput = z.output<typeof PreferenceKeyCombination>
export const MessageCompositionPatchInputConstraints = {} as const
export const MessageCompositionPatchInputInput = {}
export const MessageCompositionPatchInput = z
  .object({ convertEmoticons: z.boolean().optional(), sendShortcut: PreferenceKeyCombination.optional() })
  .strict()
export type MessageCompositionPatchInputFormInput = z.input<typeof MessageCompositionPatchInput>
export type MessageCompositionPatchInputFormOutput = z.output<typeof MessageCompositionPatchInput>
/** Text operation determining the upstream request and response format. */
export const TextModelProtocol = z
  .enum(['OPENAI_CHAT_COMPLETIONS', 'OPENAI_RESPONSES', 'ANTHROPIC_MESSAGES'])
  .describe('Text operation determining the upstream request and response format.')
export type TextModelProtocolFormInput = z.input<typeof TextModelProtocol>
export type TextModelProtocolFormOutput = z.output<typeof TextModelProtocol>
export const ModelTemplateLimitsInputConstraints = {
  context: {
    minimum: 100,
    maximum: 10000000,
    messages: {
      minimum: 'Enter between 100 and 10,000,000 tokens.',
      maximum: 'Enter between 100 and 10,000,000 tokens.',
    },
  },
  input: {
    minimum: 100,
    maximum: 10000000,
    messages: {
      minimum: 'Enter between 100 and 10,000,000 tokens.',
      maximum: 'Enter between 100 and 10,000,000 tokens.',
    },
  },
  output: {
    minimum: 100,
    maximum: 10000000,
    messages: {
      minimum: 'Enter between 100 and 10,000,000 tokens.',
      maximum: 'Enter between 100 and 10,000,000 tokens.',
    },
  },
} as const
export const ModelTemplateLimitsInputInput = {
  context: integerInput({ minimum: 100, maximum: 10000000 }),
  input: integerInput({ minimum: 100, maximum: 10000000 }),
  output: integerInput({ minimum: 100, maximum: 10000000 }),
}
export const ModelTemplateLimitsInput = z
  .object({
    context: ContextTokens.min(100, { message: 'Enter between 100 and 10,000,000 tokens.' })
      .max(10000000, { message: 'Enter between 100 and 10,000,000 tokens.' })
      .nullable()
      .optional(),
    input: ContextTokens.min(100, { message: 'Enter between 100 and 10,000,000 tokens.' })
      .max(10000000, { message: 'Enter between 100 and 10,000,000 tokens.' })
      .nullable()
      .optional(),
    output: ContextTokens.min(100, { message: 'Enter between 100 and 10,000,000 tokens.' })
      .max(10000000, { message: 'Enter between 100 and 10,000,000 tokens.' })
      .nullable()
      .optional(),
  })
  .strict()
  .superRefine((value, ctx) => {
    if (value == null) return
    for (const field of ['input', 'output'] as const) {
      if (value.context != null && value[field] != null && value[field]! > value.context)
        ctx.addIssue({
          code: 'custom',
          path: [field],
          message:
            {
              input: 'Input limit cannot exceed the context window.',
              output: 'Output limit cannot exceed the context window.',
            }[field] ?? 'Check the token limits.',
        })
    }
  })
export type ModelTemplateLimitsInputFormInput = z.input<typeof ModelTemplateLimitsInput>
export type ModelTemplateLimitsInputFormOutput = z.output<typeof ModelTemplateLimitsInput>
export const ModelTemplatePriceInputConstraints = {
  value: {
    pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
    minimum: 0,
    maximum: 10000,
    messages: {
      pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
      minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
    },
    inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
    format: 'modelPrice',
  },
} as const
export const ModelTemplatePriceInputInput = { value: stringInput(ModelTemplatePriceInputConstraints['value']) }
export const ModelTemplatePriceInput = z
  .object({
    enabled: z.boolean(),
    value: inputString({
      pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
      minimum: 0,
      maximum: 10000,
      messages: {
        pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
        minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
        maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      },
      inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
      format: 'modelPrice',
    })
      .nullable()
      .optional(),
  })
  .strict()
  .refine((value) => value == null || templatePriceObject(value), { path: ['value'], message: 'Enter a custom price.' })
export type ModelTemplatePriceInputFormInput = z.input<typeof ModelTemplatePriceInput>
export type ModelTemplatePriceInputFormOutput = z.output<typeof ModelTemplatePriceInput>
/** Capabilities declared by a catalog or configured model; not an authorization grant. */
export const ProviderModelCapability = z
  .enum([
    'TEXT',
    'VISION',
    'PDF',
    'AUDIO',
    'VIDEO',
    'IMAGE_GENERATION',
    'AUDIO_GENERATION',
    'SEARCH',
    'REASONING',
    'TOOL_CALL',
    'FORCED_TOOL_CHOICE',
    'STRUCTURED_OUTPUT',
  ])
  .describe('Capabilities declared by a catalog or configured model; not an authorization grant.')
export type ProviderModelCapabilityFormInput = z.input<typeof ProviderModelCapability>
export type ProviderModelCapabilityFormOutput = z.output<typeof ProviderModelCapability>
export const ProviderEffortMappingConstraints = {
  low: {
    minLength: 1,
    maxLength: 25,
    pattern: '^[A-Za-z0-9_-]+$',
    messages: {
      minLength: 'Enter an effort value of 1 to 25 characters.',
      maxLength: 'Enter an effort value of 1 to 25 characters.',
      pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
    },
  },
  medium: {
    minLength: 1,
    maxLength: 25,
    pattern: '^[A-Za-z0-9_-]+$',
    messages: {
      minLength: 'Enter an effort value of 1 to 25 characters.',
      maxLength: 'Enter an effort value of 1 to 25 characters.',
      pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
    },
  },
  high: {
    minLength: 1,
    maxLength: 25,
    pattern: '^[A-Za-z0-9_-]+$',
    messages: {
      minLength: 'Enter an effort value of 1 to 25 characters.',
      maxLength: 'Enter an effort value of 1 to 25 characters.',
      pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
    },
  },
  xhigh: {
    minLength: 1,
    maxLength: 25,
    pattern: '^[A-Za-z0-9_-]+$',
    messages: {
      minLength: 'Enter an effort value of 1 to 25 characters.',
      maxLength: 'Enter an effort value of 1 to 25 characters.',
      pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
    },
  },
  max: {
    minLength: 1,
    maxLength: 25,
    pattern: '^[A-Za-z0-9_-]+$',
    messages: {
      minLength: 'Enter an effort value of 1 to 25 characters.',
      maxLength: 'Enter an effort value of 1 to 25 characters.',
      pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
    },
  },
} as const
export const ProviderEffortMappingInput = {
  low: stringInput(ProviderEffortMappingConstraints['low']),
  medium: stringInput(ProviderEffortMappingConstraints['medium']),
  high: stringInput(ProviderEffortMappingConstraints['high']),
  xhigh: stringInput(ProviderEffortMappingConstraints['xhigh']),
  max: stringInput(ProviderEffortMappingConstraints['max']),
}
/** Empty object means no configured effort levels. Configured values cannot be empty. */
export const ProviderEffortMapping = z
  .object({
    low: inputString({
      minLength: 1,
      maxLength: 25,
      pattern: '^[A-Za-z0-9_-]+$',
      messages: {
        minLength: 'Enter an effort value of 1 to 25 characters.',
        maxLength: 'Enter an effort value of 1 to 25 characters.',
        pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
      },
    }).optional(),
    medium: inputString({
      minLength: 1,
      maxLength: 25,
      pattern: '^[A-Za-z0-9_-]+$',
      messages: {
        minLength: 'Enter an effort value of 1 to 25 characters.',
        maxLength: 'Enter an effort value of 1 to 25 characters.',
        pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
      },
    }).optional(),
    high: inputString({
      minLength: 1,
      maxLength: 25,
      pattern: '^[A-Za-z0-9_-]+$',
      messages: {
        minLength: 'Enter an effort value of 1 to 25 characters.',
        maxLength: 'Enter an effort value of 1 to 25 characters.',
        pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
      },
    }).optional(),
    xhigh: inputString({
      minLength: 1,
      maxLength: 25,
      pattern: '^[A-Za-z0-9_-]+$',
      messages: {
        minLength: 'Enter an effort value of 1 to 25 characters.',
        maxLength: 'Enter an effort value of 1 to 25 characters.',
        pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
      },
    }).optional(),
    max: inputString({
      minLength: 1,
      maxLength: 25,
      pattern: '^[A-Za-z0-9_-]+$',
      messages: {
        minLength: 'Enter an effort value of 1 to 25 characters.',
        maxLength: 'Enter an effort value of 1 to 25 characters.',
        pattern: 'Use letters, numbers, hyphens, or underscores for effort values.',
      },
    }).optional(),
  })
  .strict()
  .describe('Empty object means no configured effort levels. Configured values cannot be empty.')
export type ProviderEffortMappingFormInput = z.input<typeof ProviderEffortMapping>
export type ProviderEffortMappingFormOutput = z.output<typeof ProviderEffortMapping>
export const ProviderHeaderConstraints = {
  key: {
    maxLength: 128,
    pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$",
    messages: { pattern: 'Enter a valid HTTP header name.', maxLength: 'Header name must be 128 characters or fewer.' },
  },
  value: {
    pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$',
    messages: { pattern: 'Header values cannot contain newlines or unsupported characters.' },
  },
} as const
export const ProviderHeaderInput = {
  key: stringInput(ProviderHeaderConstraints['key']),
  value: stringInput(ProviderHeaderConstraints['value']),
}
export const ProviderHeader = z
  .object({
    key: inputString({
      maxLength: 128,
      pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$",
      messages: {
        pattern: 'Enter a valid HTTP header name.',
        maxLength: 'Header name must be 128 characters or fewer.',
      },
    }),
    value: inputString({
      pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$',
      messages: { pattern: 'Header values cannot contain newlines or unsupported characters.' },
    }).describe('Write-only header content; never returned by management reads.'),
  })
  .strict()
  .refine((value) => value == null || headerObject(value), { message: 'Enter a valid Authorization header value.' })
export type ProviderHeaderFormInput = z.input<typeof ProviderHeader>
export type ProviderHeaderFormOutput = z.output<typeof ProviderHeader>
export const ProviderBodyDefaultConstraints = {
  key: {
    minLength: 1,
    maxLength: 128,
    messages: {
      minLength: 'Body field names must contain 1 to 128 characters.',
      maxLength: 'Body field names must contain 1 to 128 characters.',
      'x-compallm-format': 'Use a body field name other than model, messages, input, or stream.',
      'x-compallm-input-pattern': 'Use a body field name other than model, messages, input, or stream.',
    },
    inputPattern: '^[A-Za-z0-9_.-]*$',
    format: 'bodyDefaultKey',
  },
  value: {
    minLength: 1,
    maxLength: 16384,
    messages: {
      minLength: 'Enter a JSON value of at most 16,384 characters.',
      maxLength: 'Enter a JSON value of at most 16,384 characters.',
      'x-compallm-format': 'Enter a valid JSON value.',
    },
    format: 'jsonValue',
  },
} as const
export const ProviderBodyDefaultInput = {
  key: stringInput(ProviderBodyDefaultConstraints['key']),
  value: stringInput(ProviderBodyDefaultConstraints['value']),
}
export const ProviderBodyDefault = z
  .object({
    key: inputString({
      minLength: 1,
      maxLength: 128,
      messages: {
        minLength: 'Body field names must contain 1 to 128 characters.',
        maxLength: 'Body field names must contain 1 to 128 characters.',
        'x-compallm-format': 'Use a body field name other than model, messages, input, or stream.',
        'x-compallm-input-pattern': 'Use a body field name other than model, messages, input, or stream.',
      },
      inputPattern: '^[A-Za-z0-9_.-]*$',
      format: 'bodyDefaultKey',
    }),
    value: inputString({
      minLength: 1,
      maxLength: 16384,
      messages: {
        minLength: 'Enter a JSON value of at most 16,384 characters.',
        maxLength: 'Enter a JSON value of at most 16,384 characters.',
        'x-compallm-format': 'Enter a valid JSON value.',
      },
      format: 'jsonValue',
    }),
  })
  .strict()
  .refine((value) => value == null || bodyDefaultObject(value), {
    message: 'Use the JSON value type required by the selected body field.',
  })
export type ProviderBodyDefaultFormInput = z.input<typeof ProviderBodyDefault>
export type ProviderBodyDefaultFormOutput = z.output<typeof ProviderBodyDefault>
export const ModelTemplateCustomizeInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 512,
    pattern: '^[\\x21-\\x7e]{1,512}$',
    messages: {
      minLength: 'Model identifier must contain 1 to 512 characters.',
      maxLength: 'Model identifier must contain 1 to 512 characters.',
      pattern: 'Use the exact upstream model identifier without spaces or control characters.',
    },
    trim: true,
  },
  path: {
    maxLength: 40,
    pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$',
    messages: {
      pattern: 'Enter a valid endpoint path beginning with /.',
      maxLength: 'Path must be 40 characters or fewer.',
    },
  },
  limits: {
    messages: {
      input: 'Input limit cannot exceed the context window.',
      output: 'Output limit cannot exceed the context window.',
    },
  },
  inputPrice: { messages: { 'x-compallm-object-rule': 'Enter a custom price.' } },
  outputPrice: { messages: { 'x-compallm-object-rule': 'Enter a custom price.' } },
  cachedPrice: { messages: { 'x-compallm-object-rule': 'Enter a custom price.' } },
  capabilities: {
    messages: { 'x-compallm-array-rule': 'Text models require Text; forced tool choice also requires tool calling.' },
  },
  headers: { messages: { 'x-compallm-array-rule': 'Header names must be unique, ignoring case.' } },
  bodyDefaults: { messages: { 'x-compallm-array-rule': 'Body field names must be unique.' } },
} as const
export const ModelTemplateCustomizeInputInput = {
  name: stringInput(ModelTemplateCustomizeInputConstraints['name']),
  path: stringInput(ModelTemplateCustomizeInputConstraints['path']),
}
export const ModelTemplateCustomizeInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 512,
      pattern: '^[\\x21-\\x7e]{1,512}$',
      messages: {
        minLength: 'Model identifier must contain 1 to 512 characters.',
        maxLength: 'Model identifier must contain 1 to 512 characters.',
        pattern: 'Use the exact upstream model identifier without spaces or control characters.',
      },
      trim: true,
    })
      .nullable()
      .optional(),
    protocol: TextModelProtocol.nullable().optional(),
    path: inputString({
      maxLength: 40,
      pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$',
      messages: {
        pattern: 'Enter a valid endpoint path beginning with /.',
        maxLength: 'Path must be 40 characters or fewer.',
      },
    })
      .nullable()
      .optional(),
    limits: ModelTemplateLimitsInput.nullable().optional(),
    inputPrice: ModelTemplatePriceInput.nullable().optional(),
    outputPrice: ModelTemplatePriceInput.nullable().optional(),
    cachedPrice: ModelTemplatePriceInput.nullable().optional(),
    capabilities: z
      .array(ProviderModelCapability)
      .nullable()
      .refine((value) => value == null || modelCapabilitiesArray(value), {
        message: 'Text models require Text; forced tool choice also requires tool calling.',
      })
      .optional(),
    effortMapping: ProviderEffortMapping.nullable().optional(),
    headers: z
      .array(ProviderHeader)
      .nullable()
      .refine((value) => value == null || ((entries) => headersObject({ entries }))(value), {
        message: 'Header names must be unique, ignoring case.',
      })
      .optional(),
    bodyDefaults: z
      .array(ProviderBodyDefault)
      .nullable()
      .refine((value) => value == null || bodyDefaultsArray(value), { message: 'Body field names must be unique.' })
      .optional(),
  })
  .strict()
export type ModelTemplateCustomizeInputFormInput = z.input<typeof ModelTemplateCustomizeInput>
export type ModelTemplateCustomizeInputFormOutput = z.output<typeof ModelTemplateCustomizeInput>
export const ModelTemplateCreateInputConstraints = {} as const
export const ModelTemplateCreateInputInput = {}
export const ModelTemplateCreateInput = z
  .object({ configuration: ModelTemplateCustomizeInput })
  .strict()
  .superRefine((value, ctx) => {
    for (const field of ['name', 'protocol', 'path', 'inputPrice', 'outputPrice'] as const) {
      const configured = value.configuration[field]
      const valid =
        field === 'inputPrice'
          ? value.configuration.inputPrice?.enabled && value.configuration.inputPrice.value != null
          : field === 'outputPrice'
            ? value.configuration.outputPrice?.enabled && value.configuration.outputPrice.value != null
            : configured != null
      if (!valid)
        ctx.addIssue({
          code: 'custom',
          path: ['configuration', field],
          message: {
            name: 'Enter the upstream model identifier.',
            protocol: 'Choose a text operation.',
            path: 'Enter an endpoint path.',
            inputPrice: 'Set an input price for the new model.',
            outputPrice: 'Set an output price for the new model.',
          }[field],
        })
    }
  })
export type ModelTemplateCreateInputFormInput = z.input<typeof ModelTemplateCreateInput>
export type ModelTemplateCreateInputFormOutput = z.output<typeof ModelTemplateCreateInput>
export const ModelTemplateEffectiveLimitsInputConstraints = {
  context: { minimum: 1, maximum: 4294967295 },
  input: { minimum: 1, maximum: 4294967295 },
  output: { minimum: 1, maximum: 4294967295 },
} as const
export const ModelTemplateEffectiveLimitsInputInput = {
  context: integerInput({ minimum: 1, maximum: 4294967295 }),
  input: integerInput({ minimum: 1, maximum: 4294967295 }),
  output: integerInput({ minimum: 1, maximum: 4294967295 }),
}
export const ModelTemplateEffectiveLimitsInput = z
  .object({
    context: ContextTokens.nullable().optional(),
    input: ContextTokens.nullable().optional(),
    output: ContextTokens.nullable().optional(),
  })
  .strict()
  .superRefine((value, ctx) => {
    if (value == null) return
    for (const field of ['input', 'output'] as const) {
      if (value.context != null && value[field] != null && value[field]! > value.context)
        ctx.addIssue({
          code: 'custom',
          path: [field],
          message:
            {
              input: 'Input limit cannot exceed the context window.',
              output: 'Output limit cannot exceed the context window.',
            }[field] ?? 'Check the token limits.',
        })
    }
  })
export type ModelTemplateEffectiveLimitsInputFormInput = z.input<typeof ModelTemplateEffectiveLimitsInput>
export type ModelTemplateEffectiveLimitsInputFormOutput = z.output<typeof ModelTemplateEffectiveLimitsInput>
export const ModelTemplateMatchInputConstraints = {} as const
export const ModelTemplateMatchInputInput = {}
export const ModelTemplateMatchInput = z
  .object({
    id: StringFilter.nullable().optional(),
    name: StringFilter.nullable().optional(),
    displayName: StringFilter.nullable().optional(),
  })
  .strict()
export type ModelTemplateMatchInputFormInput = z.input<typeof ModelTemplateMatchInput>
export type ModelTemplateMatchInputFormOutput = z.output<typeof ModelTemplateMatchInput>
export const ModelTemplateFilterInputConstraints = {} as const
export const ModelTemplateFilterInputInput = {}
export const ModelTemplateFilterInput = z
  .object({
    id: StringFilter.nullable().optional(),
    name: StringFilter.nullable().optional(),
    displayName: StringFilter.nullable().optional(),
    or: z.array(ModelTemplateMatchInput).nullable().optional(),
  })
  .strict()
export type ModelTemplateFilterInputFormInput = z.input<typeof ModelTemplateFilterInput>
export type ModelTemplateFilterInputFormOutput = z.output<typeof ModelTemplateFilterInput>
export const PreferenceDestination = z.enum([
  'CHAT',
  'MODELS',
  'PROVIDERS',
  'NOTEBOOKS',
  'PLUGINS',
  'STATS',
  'API_KEYS',
  'MEMBERS',
  'CONVERSATIONS',
])
export type PreferenceDestinationFormInput = z.input<typeof PreferenceDestination>
export type PreferenceDestinationFormOutput = z.output<typeof PreferenceDestination>
export const NavigationPatchInputConstraints = {} as const
export const NavigationPatchInputInput = {}
export const NavigationPatchInput = z.object({ home: PreferenceDestination.optional() }).strict()
export type NavigationPatchInputFormInput = z.input<typeof NavigationPatchInput>
export type NavigationPatchInputFormOutput = z.output<typeof NavigationPatchInput>
export const OnboardingDestination = z.enum(['CHAT', 'MODELS', 'PROVIDERS'])
export type OnboardingDestinationFormInput = z.input<typeof OnboardingDestination>
export type OnboardingDestinationFormOutput = z.output<typeof OnboardingDestination>
export const OnboardingLinkInputConstraints = {
  label: {
    minLength: 1,
    maxLength: 128,
    messages: {
      minLength: 'Enter a link label of 1 to 128 characters.',
      maxLength: 'Enter a link label of 1 to 128 characters.',
    },
  },
  href: {
    maxLength: 2048,
    messages: {
      'x-compallm-format': 'Enter a valid HTTP or HTTPS link.',
      maxLength: 'Link must be 2048 characters or fewer.',
    },
    format: 'webUrl',
  },
} as const
export const OnboardingLinkInputInput = {
  label: stringInput(OnboardingLinkInputConstraints['label']),
  href: stringInput(OnboardingLinkInputConstraints['href']),
}
export const OnboardingLinkInput = z
  .object({
    label: inputString({
      minLength: 1,
      maxLength: 128,
      messages: {
        minLength: 'Enter a link label of 1 to 128 characters.',
        maxLength: 'Enter a link label of 1 to 128 characters.',
      },
    }),
    href: inputString({
      maxLength: 2048,
      messages: {
        'x-compallm-format': 'Enter a valid HTTP or HTTPS link.',
        maxLength: 'Link must be 2048 characters or fewer.',
      },
      format: 'webUrl',
    }),
  })
  .strict()
export type OnboardingLinkInputFormInput = z.input<typeof OnboardingLinkInput>
export type OnboardingLinkInputFormOutput = z.output<typeof OnboardingLinkInput>
export const OrganizationOnboardingSettingsInputConstraints = {
  welcome: { maxLength: 4096, messages: { maxLength: 'Welcome message must be 4096 characters or fewer.' } },
  links: { maxItems: 20, messages: { maxItems: 'Use no more than 20 links.' } },
  artworkColor: {
    pattern: '^#[0-9a-fA-F]{6}$',
    messages: { pattern: 'Enter a six-digit hex color, such as #5588cc.' },
  },
} as const
export const OrganizationOnboardingSettingsInputInput = {
  welcome: stringInput(OrganizationOnboardingSettingsInputConstraints['welcome']),
  artworkColor: stringInput(OrganizationOnboardingSettingsInputConstraints['artworkColor']),
}
export const OrganizationOnboardingSettingsInput = z
  .object({
    welcome: inputString({
      maxLength: 4096,
      messages: { maxLength: 'Welcome message must be 4096 characters or fewer.' },
    }),
    links: z.array(OnboardingLinkInput).max(20, { message: 'Use no more than 20 links.' }),
    destination: OnboardingDestination,
    artworkColor: inputString({
      pattern: '^#[0-9a-fA-F]{6}$',
      messages: { pattern: 'Enter a six-digit hex color, such as #5588cc.' },
    })
      .nullable()
      .optional(),
    requireFullName: z.boolean(),
    requireAvatar: z.boolean(),
  })
  .strict()
export type OrganizationOnboardingSettingsInputFormInput = z.input<typeof OrganizationOnboardingSettingsInput>
export type OrganizationOnboardingSettingsInputFormOutput = z.output<typeof OrganizationOnboardingSettingsInput>
export const OrganizationProfileUpdateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 63,
    pattern: '^[A-Za-z0-9-]+$',
    messages: {
      minLength: 'Enter a workspace name.',
      maxLength: 'Workspace name must be 63 characters or fewer.',
      pattern: 'Use only letters, numbers, and hyphens for the workspace name.',
    },
    trim: true,
  },
} as const
export const OrganizationProfileUpdateInputInput = {
  name: stringInput(OrganizationProfileUpdateInputConstraints['name']),
}
export const OrganizationProfileUpdateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 63,
      pattern: '^[A-Za-z0-9-]+$',
      messages: {
        minLength: 'Enter a workspace name.',
        maxLength: 'Workspace name must be 63 characters or fewer.',
        pattern: 'Use only letters, numbers, and hyphens for the workspace name.',
      },
      trim: true,
    }).optional(),
    avatarUploaded: z.boolean().default(false),
  })
  .strict()
export type OrganizationProfileUpdateInputFormInput = z.input<typeof OrganizationProfileUpdateInput>
export type OrganizationProfileUpdateInputFormOutput = z.output<typeof OrganizationProfileUpdateInput>
export const OrganizationSettingsUpdateInputConstraints = {
  name: { minLength: 1, messages: { minLength: 'Enter a name.' }, trim: true },
} as const
export const OrganizationSettingsUpdateInputInput = {
  name: stringInput(OrganizationSettingsUpdateInputConstraints['name']),
}
export const OrganizationSettingsUpdateInput = z
  .object({
    name: inputString({ minLength: 1, messages: { minLength: 'Enter a name.' }, trim: true }).optional(),
    slug: inputString({}).optional(),
    fiscalMonth: inputString({}).optional(),
    welcome: inputString({}).optional(),
    announcements: inputString({}).optional(),
    home: HomeDestination.optional(),
  })
  .strict()
export type OrganizationSettingsUpdateInputFormInput = z.input<typeof OrganizationSettingsUpdateInput>
export type OrganizationSettingsUpdateInputFormOutput = z.output<typeof OrganizationSettingsUpdateInput>
export const PreferenceLightPalette = z.enum(['BASIC_WHITE', 'BLUEBERRY_WHITE', 'MARSHMALLOW_WHITE'])
export type PreferenceLightPaletteFormInput = z.input<typeof PreferenceLightPalette>
export type PreferenceLightPaletteFormOutput = z.output<typeof PreferenceLightPalette>
export const PreferenceDarkPalette = z.enum(['BASIC_DARK', 'BLUEBERRY_DARK', 'MARSHMALLOW_DARK'])
export type PreferenceDarkPaletteFormInput = z.input<typeof PreferenceDarkPalette>
export type PreferenceDarkPaletteFormOutput = z.output<typeof PreferenceDarkPalette>
export const PalettePatchInputConstraints = {} as const
export const PalettePatchInputInput = {}
export const PalettePatchInput = z
  .object({ light: PreferenceLightPalette.optional(), dark: PreferenceDarkPalette.optional() })
  .strict()
export type PalettePatchInputFormInput = z.input<typeof PalettePatchInput>
export type PalettePatchInputFormOutput = z.output<typeof PalettePatchInput>
export const PasskeyFinishInputConstraints = {
  token: { minLength: 1, maxLength: 2048 },
  id: { minLength: 1, maxLength: 2048 },
  rawId: { minLength: 1, maxLength: 2048 },
  clientDataJson: { minLength: 1, maxLength: 16384 },
  attestationObject: { maxLength: 49152 },
  authenticatorData: { maxLength: 16384 },
  signature: { maxLength: 2048 },
  userHandle: { maxLength: 256 },
} as const
export const PasskeyFinishInputInput = {
  token: stringInput(PasskeyFinishInputConstraints['token']),
  id: stringInput(PasskeyFinishInputConstraints['id']),
  rawId: stringInput(PasskeyFinishInputConstraints['rawId']),
  clientDataJson: stringInput(PasskeyFinishInputConstraints['clientDataJson']),
  attestationObject: stringInput(PasskeyFinishInputConstraints['attestationObject']),
  authenticatorData: stringInput(PasskeyFinishInputConstraints['authenticatorData']),
  signature: stringInput(PasskeyFinishInputConstraints['signature']),
  userHandle: stringInput(PasskeyFinishInputConstraints['userHandle']),
}
export const PasskeyFinishInput = z
  .object({
    purpose: AuthenticationPurpose.default('LOGIN'),
    token: inputString({ minLength: 1, maxLength: 2048 }),
    id: inputString({ minLength: 1, maxLength: 2048 }),
    rawId: inputString({ minLength: 1, maxLength: 2048 }),
    clientDataJson: inputString({ minLength: 1, maxLength: 16384 }),
    attestationObject: inputString({ maxLength: 49152 }).nullable().optional(),
    authenticatorData: inputString({ maxLength: 16384 }).nullable().optional(),
    signature: inputString({ maxLength: 2048 }).nullable().optional(),
    userHandle: inputString({ maxLength: 256 }).nullable().optional(),
    transports: z.array(inputString({})).nullable().optional(),
    delivery: CredentialDelivery.default('COOKIE'),
  })
  .strict()
export type PasskeyFinishInputFormInput = z.input<typeof PasskeyFinishInput>
export type PasskeyFinishInputFormOutput = z.output<typeof PasskeyFinishInput>
export const PasskeyRegistrationInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 64,
    messages: {
      minLength: 'Enter a passkey name of 1–64 characters.',
      maxLength: 'Enter a passkey name of 1–64 characters.',
    },
    trim: true,
  },
} as const
export const PasskeyRegistrationInputInput = { name: stringInput(PasskeyRegistrationInputConstraints['name']) }
export const PasskeyRegistrationInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 64,
      messages: {
        minLength: 'Enter a passkey name of 1–64 characters.',
        maxLength: 'Enter a passkey name of 1–64 characters.',
      },
      trim: true,
    }),
  })
  .strict()
export type PasskeyRegistrationInputFormInput = z.input<typeof PasskeyRegistrationInput>
export type PasskeyRegistrationInputFormOutput = z.output<typeof PasskeyRegistrationInput>
export const PreferenceAppearanceMode = z.enum(['SYSTEM', 'LIGHT', 'DARK'])
export type PreferenceAppearanceModeFormInput = z.input<typeof PreferenceAppearanceMode>
export type PreferenceAppearanceModeFormOutput = z.output<typeof PreferenceAppearanceMode>
export const PreferenceBadgeStyle = z.enum(['DOT', 'COUNT'])
export type PreferenceBadgeStyleFormInput = z.input<typeof PreferenceBadgeStyle>
export type PreferenceBadgeStyleFormOutput = z.output<typeof PreferenceBadgeStyle>
export const PreferenceRelativeFontSize = z.enum(['SMALLER', 'SMALL', 'DEFAULT', 'LARGE', 'LARGER'])
export type PreferenceRelativeFontSizeFormInput = z.input<typeof PreferenceRelativeFontSize>
export type PreferenceRelativeFontSizeFormOutput = z.output<typeof PreferenceRelativeFontSize>
export const PreferenceSidebarCategory = z.enum(['WORKSPACE', 'ROUTERS', 'ORGANIZATION', 'CONVERSATIONS'])
export type PreferenceSidebarCategoryFormInput = z.input<typeof PreferenceSidebarCategory>
export type PreferenceSidebarCategoryFormOutput = z.output<typeof PreferenceSidebarCategory>
export const PreferenceSidebarVisibility = z.enum(['ALWAYS', 'WHEN_BADGED', 'HIDDEN'])
export type PreferenceSidebarVisibilityFormInput = z.input<typeof PreferenceSidebarVisibility>
export type PreferenceSidebarVisibilityFormOutput = z.output<typeof PreferenceSidebarVisibility>
export const ProfileUpdateInputConstraints = {
  name: { minLength: 1, messages: { minLength: 'Enter your full name.' }, trim: true },
  username: { minLength: 1, messages: { minLength: 'Enter your username.' }, trim: true },
} as const
export const ProfileUpdateInputInput = {
  name: stringInput(ProfileUpdateInputConstraints['name']),
  username: stringInput(ProfileUpdateInputConstraints['username']),
}
export const ProfileUpdateInput = z
  .object({
    name: inputString({ minLength: 1, messages: { minLength: 'Enter your full name.' }, trim: true }).optional(),
    username: inputString({ minLength: 1, messages: { minLength: 'Enter your username.' }, trim: true }).optional(),
  })
  .strict()
export type ProfileUpdateInputFormInput = z.input<typeof ProfileUpdateInput>
export type ProfileUpdateInputFormOutput = z.output<typeof ProfileUpdateInput>
export const ProviderCatalogRefConstraints = {} as const
export const ProviderCatalogRefInput = {}
/** Opaque registry identity, independent of an edited upstream model name. */
export const ProviderCatalogRef = z
  .object({ provider: inputString({}), model: inputString({}) })
  .strict()
  .describe('Opaque registry identity, independent of an edited upstream model name.')
export type ProviderCatalogRefFormInput = z.input<typeof ProviderCatalogRef>
export type ProviderCatalogRefFormOutput = z.output<typeof ProviderCatalogRef>
export const ProviderTemplateModelInputConstraints = {} as const
export const ProviderTemplateModelInputInput = {}
export const ProviderTemplateModelInput = z.object({ id: inputString({}), protocol: TextModelProtocol }).strict()
export type ProviderTemplateModelInputFormInput = z.input<typeof ProviderTemplateModelInput>
export type ProviderTemplateModelInputFormOutput = z.output<typeof ProviderTemplateModelInput>
export const ProviderCreateFromTemplateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 120,
    messages: {
      minLength: 'Connection name must contain 1 to 120 characters.',
      maxLength: 'Connection name must contain 1 to 120 characters.',
      'x-compallm-format': 'Enter a connection name.',
    },
    format: 'nonblank',
    trim: true,
  },
  baseUrl: {
    maxLength: 512,
    messages: {
      maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
    },
    format: 'baseUrl',
  },
  apiKey: {
    minLength: 1,
    maxLength: 512,
    pattern: '^[^\\s]+$',
    messages: {
      minLength: 'Enter an API key of 1 to 512 characters.',
      maxLength: 'Enter an API key of 1 to 512 characters.',
      pattern: 'API keys cannot contain whitespace.',
    },
  },
  headers: { messages: { 'x-compallm-array-rule': 'Header names must be unique, ignoring case.' } },
  models: {
    minItems: 1,
    maxItems: 1000,
    messages: { minItems: 'Enable between 1 and 1,000 models.', maxItems: 'Enable between 1 and 1,000 models.' },
  },
} as const
export const ProviderCreateFromTemplateInputInput = {
  name: stringInput(ProviderCreateFromTemplateInputConstraints['name']),
  baseUrl: stringInput(ProviderCreateFromTemplateInputConstraints['baseUrl']),
  apiKey: stringInput(ProviderCreateFromTemplateInputConstraints['apiKey']),
}
export const ProviderCreateFromTemplateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 120,
      messages: {
        minLength: 'Connection name must contain 1 to 120 characters.',
        maxLength: 'Connection name must contain 1 to 120 characters.',
        'x-compallm-format': 'Enter a connection name.',
      },
      format: 'nonblank',
      trim: true,
    }),
    providerTemplateId: inputString({}),
    baseUrl: inputString({
      maxLength: 512,
      messages: {
        maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
        'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      },
      format: 'baseUrl',
    }),
    apiKey: inputString({
      minLength: 1,
      maxLength: 512,
      pattern: '^[^\\s]+$',
      messages: {
        minLength: 'Enter an API key of 1 to 512 characters.',
        maxLength: 'Enter an API key of 1 to 512 characters.',
        pattern: 'API keys cannot contain whitespace.',
      },
    }),
    headers: z
      .array(ProviderHeader)
      .refine((value) => value == null || ((entries) => headersObject({ entries }))(value), {
        message: 'Header names must be unique, ignoring case.',
      }),
    models: z
      .array(ProviderTemplateModelInput)
      .min(1, { message: 'Enable between 1 and 1,000 models.' })
      .max(1000, { message: 'Enable between 1 and 1,000 models.' }),
    logoUrl: inputString({}).nullable().optional(),
    logoUploadId: inputString({}).nullable().optional(),
  })
  .strict()
export type ProviderCreateFromTemplateInputFormInput = z.input<typeof ProviderCreateFromTemplateInput>
export type ProviderCreateFromTemplateInputFormOutput = z.output<typeof ProviderCreateFromTemplateInput>
export const ProviderModelConfigurationConstraints = {
  name: {
    minLength: 1,
    maxLength: 512,
    pattern: '^[\\x21-\\x7e]{1,512}$',
    messages: {
      minLength: 'Model identifier must contain 1 to 512 characters.',
      maxLength: 'Model identifier must contain 1 to 512 characters.',
      pattern: 'Use the exact upstream model identifier without spaces or control characters.',
    },
  },
  path: {
    maxLength: 40,
    pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$',
    messages: {
      pattern: 'Enter a path beginning with / using letters, numbers, hyphens, underscores, and slashes.',
      maxLength: 'Path must be 40 characters or fewer.',
    },
  },
  maxContextTokens: { minimum: 1, maximum: 4294967295 },
  maxInputTokens: { minimum: 1, maximum: 4294967295 },
  maxOutputTokens: { minimum: 1, maximum: 4294967295 },
  headers: { messages: { 'x-compallm-array-rule': 'Header names must be unique, ignoring case.' } },
  bodyDefaults: { messages: { 'x-compallm-array-rule': 'Body field names must be unique.' } },
  inputPrice: {
    pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
    minimum: 0,
    maximum: 10000,
    messages: {
      pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
      minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
    },
    inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
    format: 'modelPrice',
  },
  outputPrice: {
    pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
    minimum: 0,
    maximum: 10000,
    messages: {
      pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
      minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
    },
    inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
    format: 'modelPrice',
  },
  cachedPrice: {
    pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
    minimum: 0,
    maximum: 10000,
    messages: {
      pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
      'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
      minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
    },
    inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
    format: 'modelPrice',
  },
} as const
export const ProviderModelConfigurationInput = {
  name: stringInput(ProviderModelConfigurationConstraints['name']),
  path: stringInput(ProviderModelConfigurationConstraints['path']),
  maxContextTokens: integerInput({ minimum: 1, maximum: 4294967295 }),
  maxInputTokens: integerInput({ minimum: 1, maximum: 4294967295 }),
  maxOutputTokens: integerInput({ minimum: 1, maximum: 4294967295 }),
  inputPrice: stringInput(ProviderModelConfigurationConstraints['inputPrice']),
  outputPrice: stringInput(ProviderModelConfigurationConstraints['outputPrice']),
  cachedPrice: stringInput(ProviderModelConfigurationConstraints['cachedPrice']),
}
/** Complete configured text-model values. Decimal prices are USD per million tokens. */
export const ProviderModelConfiguration = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 512,
      pattern: '^[\\x21-\\x7e]{1,512}$',
      messages: {
        minLength: 'Model identifier must contain 1 to 512 characters.',
        maxLength: 'Model identifier must contain 1 to 512 characters.',
        pattern: 'Use the exact upstream model identifier without spaces or control characters.',
      },
    }),
    protocol: TextModelProtocol,
    path: inputString({
      maxLength: 40,
      pattern: '^/(?:[A-Za-z0-9_-][A-Za-z0-9_/-]*)?$',
      messages: {
        pattern: 'Enter a path beginning with / using letters, numbers, hyphens, underscores, and slashes.',
        maxLength: 'Path must be 40 characters or fewer.',
      },
    }),
    maxContextTokens: ContextTokens,
    contextLimitEnabled: z.boolean().default(true),
    maxInputTokens: ContextTokens.optional(),
    maxOutputTokens: ContextTokens.optional(),
    headers: z
      .array(ProviderHeader)
      .refine((value) => value == null || ((entries) => headersObject({ entries }))(value), {
        message: 'Header names must be unique, ignoring case.',
      })
      .default([]),
    bodyDefaults: z
      .array(ProviderBodyDefault)
      .refine((value) => value == null || bodyDefaultsArray(value), { message: 'Body field names must be unique.' })
      .default([]),
    enabled: z.boolean().default(true),
    effortMapping: ProviderEffortMapping.optional(),
    capabilities: z.array(ProviderModelCapability).default([]),
    catalogRef: ProviderCatalogRef.optional(),
    inputPrice: inputString({
      pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
      minimum: 0,
      maximum: 10000,
      messages: {
        pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
        minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
        maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      },
      inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
      format: 'modelPrice',
    }),
    outputPrice: inputString({
      pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
      minimum: 0,
      maximum: 10000,
      messages: {
        pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
        minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
        maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      },
      inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
      format: 'modelPrice',
    }),
    cachedPrice: inputString({
      pattern: '^0*(?:[0-9]{1,10}(?:\\.[0-9]{1,18})?|[0-9]{11}(?:\\.[0-9]{1,17})?|[0-9]{12}(?:\\.[0-9]{1,16})?)$',
      minimum: 0,
      maximum: 10000,
      messages: {
        pattern: 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-input-pattern': 'Enter a price between $0 and $10,000 per 1M tokens.',
        'x-compallm-format': 'Enter a price between $0 and $10,000 per 1M tokens.',
        minimum: 'Enter a price between $0 and $10,000 per 1M tokens.',
        maximum: 'Enter a price between $0 and $10,000 per 1M tokens.',
      },
      inputPattern: '^0*(?:[0-9]{0,10}(?:\\.[0-9]{0,18})?|[0-9]{11}(?:\\.[0-9]{0,17})?|[0-9]{12}(?:\\.[0-9]{0,16})?)$',
      format: 'modelPrice',
    }),
    inputPriceEnabled: z.boolean().default(true),
    outputPriceEnabled: z.boolean().default(true),
    cachedPriceEnabled: z.boolean().default(true),
  })
  .strict()
  .refine((value) => value == null || modelLimitsObject(value), {
    message: 'Enabled limits must be between 100 and 10,000,000 tokens and cannot exceed the context window.',
  })
  .describe('Complete configured text-model values. Decimal prices are USD per million tokens.')
export type ProviderModelConfigurationFormInput = z.input<typeof ProviderModelConfiguration>
export type ProviderModelConfigurationFormOutput = z.output<typeof ProviderModelConfiguration>
export const ProviderModelWriteConstraints = {
  id: {
    pattern: '^[1-9][0-9]*$',
    messages: {
      pattern: 'Choose a valid resource identity.',
      'x-compallm-format': 'Choose a valid resource identity.',
    },
    format: 'resourceId',
  },
  configuration: {
    messages: {
      'x-compallm-object-rule':
        'Enabled limits must be between 100 and 10,000,000 tokens and cannot exceed the context window.',
    },
  },
} as const
export const ProviderModelWriteInput = { id: stringInput(ProviderModelWriteConstraints['id']) }
/** Persisted identity is omitted for new models and forbidden during provider creation. */
export const ProviderModelWrite = z
  .object({
    id: inputString({
      pattern: '^[1-9][0-9]*$',
      messages: {
        pattern: 'Choose a valid resource identity.',
        'x-compallm-format': 'Choose a valid resource identity.',
      },
      format: 'resourceId',
    })
      .describe('Omit for a new model; supplied IDs must belong to this connection.')
      .optional(),
    configuration: ProviderModelConfiguration.describe(
      'Complete text-model configuration, including explicit token prices.',
    ),
  })
  .strict()
  .describe('Persisted identity is omitted for new models and forbidden during provider creation.')
export type ProviderModelWriteFormInput = z.input<typeof ProviderModelWrite>
export type ProviderModelWriteFormOutput = z.output<typeof ProviderModelWrite>
export const ProviderCreateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 120,
    messages: {
      minLength: 'Connection name must contain 1 to 120 characters.',
      maxLength: 'Connection name must contain 1 to 120 characters.',
      'x-compallm-format': 'Enter a connection name.',
    },
    format: 'nonblank',
    trim: true,
  },
  baseUrl: {
    maxLength: 512,
    messages: {
      maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
    },
    format: 'baseUrl',
  },
  apiKey: {
    minLength: 1,
    maxLength: 512,
    pattern: '^[^\\s]+$',
    messages: {
      minLength: 'Enter an API key of 1 to 512 characters.',
      maxLength: 'Enter an API key of 1 to 512 characters.',
      pattern: 'API keys cannot contain whitespace.',
    },
  },
  headers: { messages: { 'x-compallm-array-rule': 'Header names must be unique, ignoring case.' } },
  models: {
    minItems: 1,
    messages: {
      minItems: 'Add at least one enabled model and use unique model names and IDs.',
      'x-compallm-array-rule': 'Add at least one enabled model and use unique model names and IDs.',
    },
  },
} as const
export const ProviderCreateInputInput = {
  name: stringInput(ProviderCreateInputConstraints['name']),
  baseUrl: stringInput(ProviderCreateInputConstraints['baseUrl']),
  apiKey: stringInput(ProviderCreateInputConstraints['apiKey']),
}
/** New connection definition; credentials and header values are write-only. */
export const ProviderCreateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 120,
      messages: {
        minLength: 'Connection name must contain 1 to 120 characters.',
        maxLength: 'Connection name must contain 1 to 120 characters.',
        'x-compallm-format': 'Enter a connection name.',
      },
      format: 'nonblank',
      trim: true,
    }),
    baseUrl: inputString({
      maxLength: 512,
      messages: {
        maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
        'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      },
      format: 'baseUrl',
    }),
    apiKey: inputString({
      minLength: 1,
      maxLength: 512,
      pattern: '^[^\\s]+$',
      messages: {
        minLength: 'Enter an API key of 1 to 512 characters.',
        maxLength: 'Enter an API key of 1 to 512 characters.',
        pattern: 'API keys cannot contain whitespace.',
      },
    }),
    headers: z
      .array(ProviderHeader)
      .refine((value) => value == null || ((entries) => headersObject({ entries }))(value), {
        message: 'Header names must be unique, ignoring case.',
      }),
    models: z
      .array(ProviderModelWrite)
      .min(1, { message: 'Add at least one enabled model and use unique model names and IDs.' })
      .refine((value) => value == null || modelWritesArray(value), {
        message: 'Add at least one enabled model and use unique model names and IDs.',
      }),
  })
  .strict()
  .refine((value) => value == null || createProviderObject(value), {
    message: 'New models must not include a saved model identity.',
  })
  .describe('New connection definition; credentials and header values are write-only.')
export type ProviderCreateInputFormInput = z.input<typeof ProviderCreateInput>
export type ProviderCreateInputFormOutput = z.output<typeof ProviderCreateInput>
export const ProviderHeaderUpdateConstraints = {
  key: {
    maxLength: 128,
    pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$",
    messages: { pattern: 'Enter a valid HTTP header name.', maxLength: 'Header name must be 128 characters or fewer.' },
  },
  value: {
    pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$',
    messages: { pattern: 'Header values cannot contain newlines or unsupported characters.' },
  },
} as const
export const ProviderHeaderUpdateInput = {
  key: stringInput(ProviderHeaderUpdateConstraints['key']),
  value: stringInput(ProviderHeaderUpdateConstraints['value']),
}
/** Omit value to retain an existing header secret; new names require a value. */
export const ProviderHeaderUpdate = z
  .object({
    key: inputString({
      maxLength: 128,
      pattern: "^[!#$%&'*+.^_`|~0-9A-Za-z-]+$",
      messages: {
        pattern: 'Enter a valid HTTP header name.',
        maxLength: 'Header name must be 128 characters or fewer.',
      },
    }),
    value: inputString({
      pattern: '^[\\t\\x20-\\x7e\\x80-\\xff]*$',
      messages: { pattern: 'Header values cannot contain newlines or unsupported characters.' },
    })
      .describe('Omit to preserve the saved value for this header name. New names require a value.')
      .optional(),
  })
  .strict()
  .refine((value) => value == null || headerUpdateObject(value), {
    message: 'Enter a valid Authorization header value.',
  })
  .describe('Omit value to retain an existing header secret; new names require a value.')
export type ProviderHeaderUpdateFormInput = z.input<typeof ProviderHeaderUpdate>
export type ProviderHeaderUpdateFormOutput = z.output<typeof ProviderHeaderUpdate>
export const ProviderPatchInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 120,
    messages: {
      minLength: 'Connection name must contain 1 to 120 characters.',
      maxLength: 'Connection name must contain 1 to 120 characters.',
      'x-compallm-format': 'Enter a connection name.',
    },
    format: 'nonblank',
    trim: true,
  },
  baseUrl: {
    maxLength: 512,
    messages: {
      maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
    },
    format: 'baseUrl',
  },
  apiKey: {
    minLength: 1,
    maxLength: 512,
    pattern: '^[^\\s]+$',
    messages: {
      minLength: 'Enter an API key of 1 to 512 characters.',
      maxLength: 'Enter an API key of 1 to 512 characters.',
      pattern: 'API keys cannot contain whitespace.',
    },
  },
  headers: { messages: { 'x-compallm-array-rule': 'Header names must be unique, ignoring case.' } },
  models: {
    minItems: 1,
    messages: {
      minItems: 'Add at least one enabled model and use unique model names and IDs.',
      'x-compallm-array-rule': 'Add at least one enabled model and use unique model names and IDs.',
    },
  },
} as const
export const ProviderPatchInputInput = {
  name: stringInput(ProviderPatchInputConstraints['name']),
  baseUrl: stringInput(ProviderPatchInputConstraints['baseUrl']),
  apiKey: stringInput(ProviderPatchInputConstraints['apiKey']),
}
/** Partial update. Omitted fields are unchanged; explicit null is rejected. */
export const ProviderPatchInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 120,
      messages: {
        minLength: 'Connection name must contain 1 to 120 characters.',
        maxLength: 'Connection name must contain 1 to 120 characters.',
        'x-compallm-format': 'Enter a connection name.',
      },
      format: 'nonblank',
      trim: true,
    }).optional(),
    baseUrl: inputString({
      maxLength: 512,
      messages: {
        maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
        'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      },
      format: 'baseUrl',
    }).optional(),
    apiKey: inputString({
      minLength: 1,
      maxLength: 512,
      pattern: '^[^\\s]+$',
      messages: {
        minLength: 'Enter an API key of 1 to 512 characters.',
        maxLength: 'Enter an API key of 1 to 512 characters.',
        pattern: 'API keys cannot contain whitespace.',
      },
    })
      .describe('Omit to preserve the current credential. Empty strings and null are invalid.')
      .optional(),
    headers: z
      .array(ProviderHeaderUpdate)
      .refine((value) => value == null || ((entries) => headersObject({ entries }))(value), {
        message: 'Header names must be unique, ignoring case.',
      })
      .optional(),
    models: z
      .array(ProviderModelWrite)
      .min(1, { message: 'Add at least one enabled model and use unique model names and IDs.' })
      .refine((value) => value == null || modelWritesArray(value), {
        message: 'Add at least one enabled model and use unique model names and IDs.',
      })
      .optional(),
  })
  .strict()
  .refine((value) => value == null || providerPatchObject(value), { message: 'Change at least one connection field.' })
  .describe('Partial update. Omitted fields are unchanged; explicit null is rejected.')
export type ProviderPatchInputFormInput = z.input<typeof ProviderPatchInput>
export type ProviderPatchInputFormOutput = z.output<typeof ProviderPatchInput>
export const ProviderReplaceInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 120,
    messages: {
      minLength: 'Connection name must contain 1 to 120 characters.',
      maxLength: 'Connection name must contain 1 to 120 characters.',
      'x-compallm-format': 'Enter a connection name.',
    },
    format: 'nonblank',
    trim: true,
  },
  baseUrl: {
    maxLength: 512,
    messages: {
      maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
    },
    format: 'baseUrl',
  },
  apiKey: {
    minLength: 1,
    maxLength: 512,
    pattern: '^[^\\s]+$',
    messages: {
      minLength: 'Enter an API key of 1 to 512 characters.',
      maxLength: 'Enter an API key of 1 to 512 characters.',
      pattern: 'API keys cannot contain whitespace.',
    },
  },
  headers: { messages: { 'x-compallm-array-rule': 'Header names must be unique, ignoring case.' } },
  models: {
    minItems: 1,
    messages: {
      minItems: 'Add at least one enabled model and use unique model names and IDs.',
      'x-compallm-array-rule': 'Add at least one enabled model and use unique model names and IDs.',
    },
  },
} as const
export const ProviderReplaceInputInput = {
  name: stringInput(ProviderReplaceInputConstraints['name']),
  baseUrl: stringInput(ProviderReplaceInputConstraints['baseUrl']),
  apiKey: stringInput(ProviderReplaceInputConstraints['apiKey']),
}
/** Full non-secret replacement. Omitting apiKey preserves the saved credential. */
export const ProviderReplaceInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 120,
      messages: {
        minLength: 'Connection name must contain 1 to 120 characters.',
        maxLength: 'Connection name must contain 1 to 120 characters.',
        'x-compallm-format': 'Enter a connection name.',
      },
      format: 'nonblank',
      trim: true,
    }),
    baseUrl: inputString({
      maxLength: 512,
      messages: {
        maxLength: 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
        'x-compallm-format': 'Enter an HTTP or HTTPS server URL without credentials, query, or fragment.',
      },
      format: 'baseUrl',
    }),
    apiKey: inputString({
      minLength: 1,
      maxLength: 512,
      pattern: '^[^\\s]+$',
      messages: {
        minLength: 'Enter an API key of 1 to 512 characters.',
        maxLength: 'Enter an API key of 1 to 512 characters.',
        pattern: 'API keys cannot contain whitespace.',
      },
    })
      .describe('Omit to preserve the current credential. Empty strings and null are invalid.')
      .optional(),
    headers: z
      .array(ProviderHeaderUpdate)
      .refine((value) => value == null || ((entries) => headersObject({ entries }))(value), {
        message: 'Header names must be unique, ignoring case.',
      }),
    models: z
      .array(ProviderModelWrite)
      .min(1, { message: 'Add at least one enabled model and use unique model names and IDs.' })
      .refine((value) => value == null || modelWritesArray(value), {
        message: 'Add at least one enabled model and use unique model names and IDs.',
      }),
  })
  .strict()
  .describe('Full non-secret replacement. Omitting apiKey preserves the saved credential.')
export type ProviderReplaceInputFormInput = z.input<typeof ProviderReplaceInput>
export type ProviderReplaceInputFormOutput = z.output<typeof ProviderReplaceInput>
export const ProviderTemplateMatchInputConstraints = {} as const
export const ProviderTemplateMatchInputInput = {}
export const ProviderTemplateMatchInput = z
  .object({ id: StringFilter.nullable().optional(), name: StringFilter.nullable().optional() })
  .strict()
export type ProviderTemplateMatchInputFormInput = z.input<typeof ProviderTemplateMatchInput>
export type ProviderTemplateMatchInputFormOutput = z.output<typeof ProviderTemplateMatchInput>
export const ProviderTemplateFilterInputConstraints = {} as const
export const ProviderTemplateFilterInputInput = {}
export const ProviderTemplateFilterInput = z
  .object({
    id: StringFilter.nullable().optional(),
    name: StringFilter.nullable().optional(),
    or: z.array(ProviderTemplateMatchInput).nullable().optional(),
  })
  .strict()
export type ProviderTemplateFilterInputFormInput = z.input<typeof ProviderTemplateFilterInput>
export type ProviderTemplateFilterInputFormOutput = z.output<typeof ProviderTemplateFilterInput>
export const ProviderTemplateOrderInputConstraints = {} as const
export const ProviderTemplateOrderInputInput = {}
export const ProviderTemplateOrderInput = z
  .object({ name: OrderByDirection.nullable().optional(), id: OrderByDirection.nullable().optional() })
  .strict()
export type ProviderTemplateOrderInputFormInput = z.input<typeof ProviderTemplateOrderInput>
export type ProviderTemplateOrderInputFormOutput = z.output<typeof ProviderTemplateOrderInput>
export const ProvidersCreateInputConstraints = {} as const
export const ProvidersCreateInputInput = {}
/** Atomic batch of new provider connections for the verified organization. */
export const ProvidersCreateInput = z
  .object({ providers: z.array(ProviderCreateInput) })
  .strict()
  .refine((value) => value == null || createProvidersObject(value), {
    message: 'Add at least one connection and use unique connection names.',
  })
  .describe('Atomic batch of new provider connections for the verified organization.')
export type ProvidersCreateInputFormInput = z.input<typeof ProvidersCreateInput>
export type ProvidersCreateInputFormOutput = z.output<typeof ProvidersCreateInput>
export const ReportingScheduleInputConstraints = {} as const
export const ReportingScheduleInputInput = {}
export const ReportingScheduleInput = z.object({ timezone: inputString({}) }).strict()
export type ReportingScheduleInputFormInput = z.input<typeof ReportingScheduleInput>
export type ReportingScheduleInputFormOutput = z.output<typeof ReportingScheduleInput>
export const RoleCreateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 64,
    messages: { minLength: 'Name is required.', maxLength: 'Name must be 64 characters or fewer.' },
    trim: true,
  },
  description: { maxLength: 128, messages: { maxLength: 'Description is too long.' } },
  cedarSource: {
    maxLength: 10192,
    messages: { maxLength: 'Policy must be 10192 characters or fewer.' },
    cedarCompact: true,
  },
} as const
export const RoleCreateInputInput = {
  name: stringInput(RoleCreateInputConstraints['name']),
  description: stringInput(RoleCreateInputConstraints['description']),
  cedarSource: stringInput(RoleCreateInputConstraints['cedarSource']),
}
export const RoleCreateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 64,
      messages: { minLength: 'Name is required.', maxLength: 'Name must be 64 characters or fewer.' },
      trim: true,
    }),
    description: inputString({ maxLength: 128, messages: { maxLength: 'Description is too long.' } }),
    cedarSource: inputString({
      maxLength: 10192,
      messages: { maxLength: 'Policy must be 10192 characters or fewer.' },
      cedarCompact: true,
    }).describe('Comments and unnecessary whitespace are removed before the length limit is checked.'),
  })
  .strict()
export type RoleCreateInputFormInput = z.input<typeof RoleCreateInput>
export type RoleCreateInputFormOutput = z.output<typeof RoleCreateInput>
export const RoleUpdateInputConstraints = {
  name: {
    minLength: 1,
    maxLength: 64,
    messages: { minLength: 'Name is required.', maxLength: 'Name must be 64 characters or fewer.' },
    trim: true,
  },
  description: { maxLength: 128, messages: { maxLength: 'Description is too long.' } },
  cedarSource: {
    maxLength: 10192,
    messages: { maxLength: 'Policy must be 10192 characters or fewer.' },
    cedarCompact: true,
  },
} as const
export const RoleUpdateInputInput = {
  name: stringInput(RoleUpdateInputConstraints['name']),
  description: stringInput(RoleUpdateInputConstraints['description']),
  cedarSource: stringInput(RoleUpdateInputConstraints['cedarSource']),
}
export const RoleUpdateInput = z
  .object({
    name: inputString({
      minLength: 1,
      maxLength: 64,
      messages: { minLength: 'Name is required.', maxLength: 'Name must be 64 characters or fewer.' },
      trim: true,
    })
      .nullable()
      .optional(),
    description: inputString({ maxLength: 128, messages: { maxLength: 'Description is too long.' } })
      .nullable()
      .optional(),
    cedarSource: inputString({
      maxLength: 10192,
      messages: { maxLength: 'Policy must be 10192 characters or fewer.' },
      cedarCompact: true,
    })
      .nullable()
      .describe('Comments and unnecessary whitespace are removed before the length limit is checked.')
      .optional(),
  })
  .strict()
export type RoleUpdateInputFormInput = z.input<typeof RoleUpdateInput>
export type RoleUpdateInputFormOutput = z.output<typeof RoleUpdateInput>
export const SidebarLayoutSectionInputConstraints = {} as const
export const SidebarLayoutSectionInputInput = {}
export const SidebarLayoutSectionInput = z
  .object({ category: PreferenceSidebarCategory, items: z.array(PreferenceDestination) })
  .strict()
export type SidebarLayoutSectionInputFormInput = z.input<typeof SidebarLayoutSectionInput>
export type SidebarLayoutSectionInputFormOutput = z.output<typeof SidebarLayoutSectionInput>
export const SidebarVisibilityPatchInputConstraints = {} as const
export const SidebarVisibilityPatchInputInput = {}
export const SidebarVisibilityPatchInput = z
  .object({ item: PreferenceDestination, visibility: PreferenceSidebarVisibility })
  .strict()
export type SidebarVisibilityPatchInputFormInput = z.input<typeof SidebarVisibilityPatchInput>
export type SidebarVisibilityPatchInputFormOutput = z.output<typeof SidebarVisibilityPatchInput>
export const SidebarPatchInputConstraints = {} as const
export const SidebarPatchInputInput = {}
/** Order replaces ordering while preserving visibility; visibility patches are keyed by destination. */
export const SidebarPatchInput = z
  .object({
    layout: z.array(SidebarLayoutSectionInput).optional(),
    order: z.array(PreferenceDestination).optional(),
    visibility: z.array(SidebarVisibilityPatchInput).optional(),
    badgeStyle: PreferenceBadgeStyle.optional(),
  })
  .strict()
  .describe('Order replaces ordering while preserving visibility; visibility patches are keyed by destination.')
export type SidebarPatchInputFormInput = z.input<typeof SidebarPatchInput>
export type SidebarPatchInputFormOutput = z.output<typeof SidebarPatchInput>
export const SpendingLimitInputConstraints = {
  amount: { pattern: '^[0-9]+(?:\\.[0-9]+)?$', messages: { pattern: 'Enter a nonnegative amount.' } },
} as const
export const SpendingLimitInputInput = { amount: stringInput(SpendingLimitInputConstraints['amount']) }
export const SpendingLimitInput = z
  .object({
    amount: inputString({ pattern: '^[0-9]+(?:\\.[0-9]+)?$', messages: { pattern: 'Enter a nonnegative amount.' } })
      .nullable()
      .optional(),
  })
  .strict()
export type SpendingLimitInputFormInput = z.input<typeof SpendingLimitInput>
export type SpendingLimitInputFormOutput = z.output<typeof SpendingLimitInput>
export const ThemePatchInputConstraints = {} as const
export const ThemePatchInputInput = {}
export const ThemePatchInput = z
  .object({ mode: PreferenceAppearanceMode.optional(), palettes: PalettePatchInput.optional() })
  .strict()
export type ThemePatchInputFormInput = z.input<typeof ThemePatchInput>
export type ThemePatchInputFormOutput = z.output<typeof ThemePatchInput>
export const TypographyPatchInputConstraints = {} as const
export const TypographyPatchInputInput = {}
export const TypographyPatchInput = z
  .object({ family: PreferenceFontFamily.optional(), size: PreferenceRelativeFontSize.optional() })
  .strict()
export type TypographyPatchInputFormInput = z.input<typeof TypographyPatchInput>
export type TypographyPatchInputFormOutput = z.output<typeof TypographyPatchInput>
export const UserPreferencesPatchInputConstraints = {} as const
export const UserPreferencesPatchInputInput = {}
/** Partial personal preferences. Omitted fields are preserved; explicit null is rejected. */
export const UserPreferencesPatchInput = z
  .object({
    navigation: NavigationPatchInput.optional(),
    sidebar: SidebarPatchInput.optional(),
    memberDisplay: MemberDisplayPatchInput.optional(),
    calendar: CalendarPatchInput.optional(),
    theme: ThemePatchInput.optional(),
    typography: TypographyPatchInput.optional(),
    accessibility: AccessibilityPatchInput.optional(),
    messageComposition: MessageCompositionPatchInput.optional(),
    codeAppearance: CodeAppearancePatchInput.optional(),
  })
  .strict()
  .describe('Partial personal preferences. Omitted fields are preserved; explicit null is rejected.')
export type UserPreferencesPatchInputFormInput = z.input<typeof UserPreferencesPatchInput>
export type UserPreferencesPatchInputFormOutput = z.output<typeof UserPreferencesPatchInput>
export const UserProfileUpdateInputConstraints = {
  displayName: {
    minLength: 0,
    maxLength: 256,
    messages: { maxLength: 'Full name must be 256 characters or fewer.' },
    trim: true,
  },
  username: {
    minLength: 1,
    maxLength: 256,
    pattern: "^(?!\\.)(?!.*\\.\\.)(?!.*\\.$)[A-Za-z0-9.!#$%&'*+/=?^_`{|}~\\-]+$",
    messages: {
      minLength: 'Enter your username.',
      maxLength: 'Username must be 256 characters or fewer.',
      pattern: 'Use a valid username without leading, trailing, or consecutive dots.',
    },
    trim: true,
    asciiLowercase: true,
  },
} as const
export const UserProfileUpdateInputInput = {
  displayName: stringInput(UserProfileUpdateInputConstraints['displayName']),
  username: stringInput(UserProfileUpdateInputConstraints['username']),
}
export const UserProfileUpdateInput = z
  .object({
    displayName: inputString({
      minLength: 0,
      maxLength: 256,
      messages: { maxLength: 'Full name must be 256 characters or fewer.' },
      trim: true,
    }).optional(),
    username: inputString({
      minLength: 1,
      maxLength: 256,
      pattern: "^(?!\\.)(?!.*\\.\\.)(?!.*\\.$)[A-Za-z0-9.!#$%&'*+/=?^_`{|}~\\-]+$",
      messages: {
        minLength: 'Enter your username.',
        maxLength: 'Username must be 256 characters or fewer.',
        pattern: 'Use a valid username without leading, trailing, or consecutive dots.',
      },
      trim: true,
      asciiLowercase: true,
    }).optional(),
    avatarUploaded: z.boolean().default(false),
  })
  .strict()
export type UserProfileUpdateInputFormInput = z.input<typeof UserProfileUpdateInput>
export type UserProfileUpdateInputFormOutput = z.output<typeof UserProfileUpdateInput>
