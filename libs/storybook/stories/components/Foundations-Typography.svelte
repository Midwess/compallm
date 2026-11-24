<script lang="ts">
  import { FormFieldLabel } from '../../src/components/forms'
  import { HelperText } from '../../src/components/feedback'
  import { SettingsPageTitle, SettingsSectionTitle } from '../../src/components/settings'
</script>

<div class="grid max-w-3xl gap-8 p-6">
  <SettingsPageTitle>Page title · 24 / 32 · Medium</SettingsPageTitle><SettingsSectionTitle>
    Section title · 15 / 23 · Medium
  </SettingsSectionTitle><FormFieldLabel>Field label · 14 / 21 · Medium</FormFieldLabel>
  <p class="text-xsm leading-5">Body text · 13 / 20 · Regular</p>
  <HelperText>Helper text · 12 / 18 · Book (450)</HelperText>
  <div class="grid gap-3 text-xsm leading-5">
    <p class="font-thin">Thin · 100</p>
    <p class="font-extralight">Extra light · 200</p>
    <p class="font-light">Light · 300</p>
    <p class="font-normal">Regular · 400</p>
    <p class="font-book">Book · 450</p>
    <p class="font-medium">Medium · 500</p>
    <p class="font-semibold">Semibold · 600</p>
    <p class="font-bold">Bold · 700</p>
    <p class="font-extrabold">Extra bold · 800</p>
    <p class="font-black">Black · 900</p>
    <p class="font-[475]">Intermediate weight · 475</p>
    <p class="font-book italic">Book italic · 450</p>
    <p class="font-semibold italic">Semibold italic · 600</p>
    <p lang="vi" class="font-book">Tiếng Việt · Cài đặt giao diện và tùy chọn cá nhân</p>
  </div>
</div>
