import type { HTMLAttributes, TableHTMLAttributes, ThHTMLAttributes, TdHTMLAttributes } from 'react'
import { cx } from './shared'
function Root({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('w-full overflow-x-auto', className)} />
}
function Content({ className, ...props }: TableHTMLAttributes<HTMLTableElement>) {
  return <table {...props} className={cx('w-full table-fixed text-left text-xs', className)} />
}
function Header({ className, ...props }: HTMLAttributes<HTMLTableSectionElement>) {
  return <thead {...props} className={cx('h-8 text-muted [&>tr]:h-8', className)} />
}
function Body(props: HTMLAttributes<HTMLTableSectionElement>) {
  return <tbody {...props} />
}
function Row({ className, ...props }: HTMLAttributes<HTMLTableRowElement>) {
  return <tr {...props} className={cx('h-12 border-b border-divider last:border-b-0', className)} />
}
function Head({ className, ...props }: ThHTMLAttributes<HTMLTableCellElement>) {
  return <th scope="col" {...props} className={cx('px-2 font-normal', className)} />
}
function Cell({ className, ...props }: TdHTMLAttributes<HTMLTableCellElement>) {
  return <td {...props} className={cx('px-2', className)} />
}
function Empty({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      {...props}
      className={cx('flex min-h-20 items-center justify-center py-4 text-center text-xs text-muted', className)}
    />
  )
}
export const Table = Object.assign(Root, { Content, Header, Body, Row, Head, Cell, Empty })
