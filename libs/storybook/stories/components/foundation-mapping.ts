const foundationPage = 'e97f7e0e-a80b-80ee-8008-da6e68461359'
const file = '893a03ee-1999-8101-8008-da5fd989e5a1'
const team = '893a03ee-1999-8101-8008-da5fd2d46656'

export function penpotFoundationUrl(board: string) {
  return `http://localhost:9001/#/workspace?team-id=${team}&file-id=${file}&page-id=${foundationPage}&board-id=${board}`
}

export const foundationFamilies = [
  {
    name: '01 · Foundations',
    board: '5f0632c6-6f9f-80b5-8008-dc0d6d2d6b97',
    source: 'styles/theme.css',
    story: 'components-foundations--colors',
  },
  {
    name: '02 · Text fields',
    board: '5f0632c6-6f9f-80b5-8008-dc093d0a28ce',
    source: 'components/fields.tsx',
    story: 'components-text-fields--text-field',
  },
  {
    name: '03 · Buttons & links',
    board: '5f0632c6-6f9f-80b5-8008-dc09eaab5265',
    source: 'components/buttons.tsx',
    story: 'components-buttons-links--button',
  },
  {
    name: '04 · Dropdowns',
    board: '5f0632c6-6f9f-80b5-8008-dc0a2844179c',
    source: 'components/dropdowns.tsx',
    story: 'components-dropdowns--dropdown',
  },
  {
    name: '05 · Selection controls',
    board: '5f0632c6-6f9f-80b5-8008-dc0b2a6cc877',
    source: 'components/selection.tsx',
    story: 'components-selection-controls--switch',
  },
  {
    name: '06 · Feedback & overlays',
    board: '5f0632c6-6f9f-80b5-8008-dc0c301b214e',
    source: 'components/feedback.tsx, dialog.tsx, popup.tsx',
    story: 'components-feedback-overlays--helper-text',
  },
  {
    name: '07 · Content & code',
    board: '5f0632c6-6f9f-80b5-8008-dc0d326d4a9e',
    source: 'components/content.tsx',
    story: 'components-content-code--settings-row',
  },
  {
    name: '08 · Form primitives',
    board: '5f0632c6-6f9f-80b5-8008-dc0e5814815f',
    source: 'components/forms.tsx, form-field.tsx',
    story: 'components-form-primitives--form-field-label',
  },
  {
    name: '09 · Navigation controls',
    board: '5f0632c6-6f9f-80b5-8008-dc1bbcd0471d',
    source: 'components/navigation.tsx, page-header.tsx',
    story: 'components-navigation-controls--toolbar-action',
  },
  {
    name: '10 · Data primitives',
    board: '5f0632c6-6f9f-80b5-8008-dc1bbcfe6b7f',
    source: 'components/data.tsx, table.tsx',
    story: 'components-data-primitives--project-table-row',
  },
  {
    name: '11 · Icons · Phosphor',
    board: 'b892fddc-5afa-80f9-8008-dd551fc73674',
    source: '@phosphor-icons/react',
    story: 'components-icons--phosphor',
  },
  {
    name: '12 · Section surfaces',
    board: '6aa90b21-8873-809e-8008-dd5bccd23876',
    source: 'components/surfaces.tsx',
    story: 'components-section-surfaces--section-surface',
  },
  {
    name: '13 · Settings composition',
    board: '5c696641-0e0d-80da-8008-e74e57be1fd7',
    source: 'components/settings.tsx',
    story: 'components-settings-composition--composition',
  },
] as const
