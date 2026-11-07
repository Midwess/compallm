import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import {
  CreateProvidersRequest,
  ReplaceProviderRequest,
  PatchProviderRequest,
} from "../../libs/contract/dist/validation.js";
const schemas = {
  create: CreateProvidersRequest,
  replace: ReplaceProviderRequest,
  patch: PatchProviderRequest,
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

test("OpenAPI exposes all eight schema-only operations and protects read responses", async () => {
  const document = JSON.parse(
    await readFile(
      new URL("../../portal/openapi.json", import.meta.url),
      "utf8",
    ),
  );
  const operations = [
    ["get", "/api/provider-registry", "listProviderRegistry"],
    ["get", "/api/provider-registry/{registryId}", "getProviderRegistry"],
    ["get", "/api/providers", "listProviders"],
    ["get", "/api/providers/{providerId}", "getProvider"],
    ["post", "/api/providers", "createProviders"],
    ["put", "/api/providers/{providerId}", "replaceProvider"],
    ["patch", "/api/providers/{providerId}", "patchProvider"],
    ["delete", "/api/providers/{providerId}", "deleteProvider"],
  ];
  for (const [method, path, operationId] of operations) {
    const operation = document.paths[path][method];
    assert.equal(operation.operationId, operationId);
    assert.match(operation.description, /no runtime handler/);
    assert.deepEqual(operation.security, [
      { bearerAuth: [] },
      { accessCookie: [] },
    ]);
  }
  assert.equal(
    document.paths["/api/providers/{providerId}"].delete.responses["204"]
      .content,
    undefined,
  );
  assert.equal(
    document.components.schemas.ProviderDetailResponse.properties.apiKey,
    undefined,
  );
  assert.equal(
    document.components.schemas.ProviderHeaderMetadata.properties.value,
    undefined,
  );
});
