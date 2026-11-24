import AlertContent from './AlertDialogContent.svelte'
import Cancel from './DialogCancel.svelte'
import Root from './DialogRoot.svelte'
import Trigger from './DialogTrigger.svelte'
import Content from './DialogContent.svelte'
import Close from './DialogClose.svelte'
import Header from './DialogHeader.svelte'
import Body from './DialogBody.svelte'
import Footer from './DialogFooter.svelte'
import Title from './DialogTitle.svelte'
import Description from './DialogDescription.svelte'
import Discard from './DialogDiscard.svelte'
export const Dialog: {
  Root: typeof Root
  Trigger: typeof Trigger
  Content: typeof Content
  Close: typeof Close
  Header: typeof Header
  Body: typeof Body
  Footer: typeof Footer
  Title: typeof Title
  Description: typeof Description
  Discard: typeof Discard
  Cancel: typeof Cancel
} = { Root, Trigger, Content, Close, Header, Body, Footer, Title, Description, Discard, Cancel }
export const AlertDialog: {
  Root: typeof Root
  Trigger: typeof Trigger
  Content: typeof AlertContent
  Close: typeof Close
  Cancel: typeof Cancel
  Header: typeof Header
  Body: typeof Body
  Footer: typeof Footer
  Title: typeof Title
  Description: typeof Description
} = { Root, Trigger, Content: AlertContent, Close, Cancel, Header, Body, Footer, Title, Description }
