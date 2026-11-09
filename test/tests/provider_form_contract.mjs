import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import {
  ProvidersCreateInput,
  ProviderReplaceInput,
  ProviderPatchInput,
} from "../../libs/contract/dist/validation.js";
const schemas = {
  create: ProvidersCreateInput,
  replace: ProviderReplaceInput,
  patch: ProviderPatchInput,
};
const fixtures = JSON.parse(
  await readFile(
    new URL("./provider-api-fixtures.json", import.meta.url),
    "utf8",
  ),
);
for (const fixture of fixtures) {
  test(fixture.name, () =>
    assert.equal(
      schemas[fixture.kind].safeParse(fixture.value).success,
      fixture.valid,
    ),
  );
}
