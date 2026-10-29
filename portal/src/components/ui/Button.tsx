import type { ButtonHTMLAttributes } from 'react'

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost'
}

/** Shared portal button. Theme values are provisional during design review. */
export function Button({ variant = 'primary', type = 'button', className = '', ...props }: ButtonProps) {
  return <button {...props} type={type} className={`ui-button ui-button--${variant} ${className}`} />
}
