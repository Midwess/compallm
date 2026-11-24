import Empty from './TableEmpty.svelte'
import Root from './TableRoot.svelte'
import Content from './TableContent.svelte'
import Header from './TableHeader.svelte'
import Body from './TableBody.svelte'
import Footer from './TableFooter.svelte'
import Row from './TableRow.svelte'
import Head from './TableHead.svelte'
import Cell from './TableCell.svelte'
import Caption from './TableCaption.svelte'
import Category from './TableCategory.svelte'
import LoadMore from './TableLoadMore.svelte'
export const Table: {
  Root: typeof Root
  Content: typeof Content
  Header: typeof Header
  Body: typeof Body
  Footer: typeof Footer
  Row: typeof Row
  Head: typeof Head
  Cell: typeof Cell
  Caption: typeof Caption
  Category: typeof Category
  LoadMore: typeof LoadMore
  Empty: typeof Empty
} = { Root, Content, Header, Body, Footer, Row, Head, Cell, Caption, Category, LoadMore, Empty }
export type { TableColumn } from './table-context'
