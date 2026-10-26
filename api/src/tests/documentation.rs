use std::fs;

use serde_json::json;

use crate::{controllers, documentation::ApiDocumentation};

#[test]
fn current_contract_describes_operations_fields_and_safe_examples() {
    let document = serde_json::to_value(controllers::openapi()).unwrap();
    let pages = ApiDocumentation::new(&document).render().unwrap();
    assert_eq!(pages.len(), 6);
    let me = &pages["auth/me/get.md"];
    assert!(me.contains("bearerAuth"));
    assert!(me.contains("accessCookie"));
    assert!(!me.contains("Public endpoint"));
    let email = &pages["auth/signup/email/post.md"];
    assert!(email.contains("Request an email signup code."));
    assert!(email.contains("Public endpoint"));
    assert!(email.contains("reader@example.com"));
    assert!(email.contains("not a user access credential"));
    assert!(email.contains("### 503"));
    assert!(email.contains("../../../index.md"));
    assert!(email.contains("available: false"));
    assert!(!email.lines().any(|line| line.ends_with(' ')));
    assert!(email.contains("\"message\": \"Email delivery is unavailable\""));
    let confirmation = &pages["auth/signup/email/confirmation/otp/post.md"];
    assert!(confirmation.contains("Delivery defaults to cookie"));
    assert!(confirmation.contains("never stored in the signed JWTs"));
    assert!(confirmation.contains("accessToken"));
    assert!(confirmation.contains("### 409"));
    assert!(confirmation.contains("example-email-challenge"));
    assert!(pages["index.md"].contains("auth/signup/email/confirmation/otp/post.md"));
}

#[test]
fn path_parameters_methods_security_and_availability_are_preserved() {
    let document = json!({
        "security": [{"session": []}],
        "components": {"securitySchemes": {"session": {"type": "apiKey", "in": "cookie", "name": "session"}}},
        "paths": {"/api/users/{userId}": {
            "parameters": [{"name": "userId", "in": "path", "required": true, "description": "User identifier", "schema": {"type": "integer"}}],
            "get": {"summary": "Read a user", "x-docs-available": true, "responses": {"200": {"description": "User details"}}},
            "post": {"summary": "Update a user", "security": [], "responses": {"204": {"description": "Updated"}}}
        }}
    });
    let pages = ApiDocumentation::new(&document).render().unwrap();
    let get = &pages["users/_userId/get.md"];
    assert!(get.contains("User identifier"));
    assert!(get.contains("session"));
    assert!(!get.contains("Public endpoint"));
    assert!(get.contains("available: true"));
    assert!(get.contains("## Example request"));
    assert!(pages["users/_userId/post.md"].contains("Public endpoint"));
    assert!(pages["users/_userId/post.md"].contains("available: false"));
    assert!(pages["index.md"].contains("available: true"));
}

#[test]
fn regeneration_removes_only_owned_stale_pages_and_is_deterministic() {
    let directory = tempfile::tempdir().unwrap();
    let handwritten = directory.path().join("adr.md");
    fs::write(&handwritten, "# Handwritten decisions\n").unwrap();
    let original = json!({"paths": {"/api/old": {"get": {"responses": {}}}}});
    ApiDocumentation::new(&original)
        .generate(directory.path())
        .unwrap();
    let next = json!({"paths": {"/api/new": {"post": {"responses": {}}}}});
    let docs = ApiDocumentation::new(&next);
    docs.generate(directory.path()).unwrap();
    assert!(!directory.path().join("old/get.md").exists());
    let first = fs::read_to_string(directory.path().join("new/post.md")).unwrap();
    docs.generate(directory.path()).unwrap();
    assert_eq!(
        first,
        fs::read_to_string(directory.path().join("new/post.md")).unwrap()
    );
    assert_eq!(
        fs::read_to_string(handwritten).unwrap(),
        "# Handwritten decisions\n"
    );
}

#[test]
fn handwritten_collisions_fail_before_writing_or_removing_pages() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("index.md"), "Handwritten guide").unwrap();
    let document = json!({"paths": {"/api/new": {"post": {"responses": {}}}}});
    let error = ApiDocumentation::new(&document)
        .generate(directory.path())
        .unwrap_err();
    assert!(error.to_string().contains("handwritten"));
    assert!(!directory.path().join("new/post.md").exists());
    assert_eq!(
        fs::read_to_string(directory.path().join("index.md")).unwrap(),
        "Handwritten guide"
    );
}

#[test]
fn unsafe_or_colliding_routes_fail_instead_of_escaping_or_overwriting() {
    for route in [
        "/api/../outside",
        "/api/a/../../outside",
        "/api/%2e%2e/outside",
        "/api//outside",
    ] {
        let document = json!({"paths": {route: {"get": {"responses": {}}}}});
        assert!(
            ApiDocumentation::new(&document).render().is_err(),
            "accepted {route}"
        );
    }
    let document = json!({"paths": {
        "/api/users/{id}": {"get": {"responses": {}}},
        "/api/users/_id": {"get": {"responses": {}}}
    }});
    assert!(
        ApiDocumentation::new(&document)
            .render()
            .unwrap_err()
            .to_string()
            .contains("multiple operations")
    );
}

#[test]
fn missing_references_fail_and_recursive_schemas_terminate() {
    let missing = json!({"paths": {"/api/example": {"get": {"responses": {"200": {"$ref": "#/components/responses/Missing"}}}}}});
    assert!(ApiDocumentation::new(&missing).render().is_err());
    let recursive = json!({
        "paths": {"/api/nodes": {"get": {"responses": {"200": {"description": "Nodes", "content": {"application/json": {"schema": {"$ref": "#/components/schemas/Node"}}}}}}}},
        "components": {"schemas": {"Node": {"type": "object", "properties": {"child": {"$ref": "#/components/schemas/Node"}}}}}
    });
    assert!(ApiDocumentation::new(&recursive).render().is_ok());
}
