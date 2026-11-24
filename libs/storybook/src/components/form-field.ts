import Root from './FormFieldRoot.svelte'
import Label from './FieldLabel.svelte'
import Input from './FormFieldInput.svelte'
import PrefixedInput from './PrefixedTextField.svelte'
import Hint from './FormFieldHint.svelte'
import Error from './FormError.svelte'
export const FormField: {
  Root: typeof Root
  Label: typeof Label
  Input: typeof Input
  PrefixedInput: typeof PrefixedInput
  Hint: typeof Hint
  Error: typeof Error
} = { Root, Label, Input, PrefixedInput, Hint, Error }
