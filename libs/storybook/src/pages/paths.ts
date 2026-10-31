export const pagePaths = {
  signup: '/signup',
  signin: '/signin',
} as const

export type AuthPagePath = (typeof pagePaths)[keyof typeof pagePaths]
