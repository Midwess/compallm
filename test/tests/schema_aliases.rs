use compallm_postgres::codegen::SchemaAliases;
use std::fs;

#[test]
fn aliases_use_only_registered_model_modules_and_singular_names() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::write(root.join("mod.rs"), "pub mod users; pub mod organizations; pub mod people; pub mod prelude; pub mod sea_orm_active_enums; pub mod r#type;").unwrap();
    for name in ["users", "organizations", "people", "type", "stale_table"] {
        fs::write(
            root.join(format!("{name}.rs")),
            "pub struct Model { pub id: i64 }",
        )
        .unwrap();
    }
    fs::write(
        root.join("prelude.rs"),
        "pub use super::users::Entity as Users;",
    )
    .unwrap();
    fs::write(
        root.join("sea_orm_active_enums.rs"),
        "pub enum UserStatus { Active } pub enum Role { Admin } enum Internal { Hidden }",
    )
    .unwrap();
    SchemaAliases::generate(root).unwrap();
    let aliases = fs::read_to_string(root.join("mod.rs")).unwrap();
    assert!(aliases.contains("mod users;"));
    assert!(!aliases.contains("pub mod"));
    assert!(aliases.contains("pub use self::users::Model as UserSchema;"));
    assert!(aliases.contains("pub use self::organizations::Model as OrganizationSchema;"));
    assert!(aliases.contains("pub use self::people::Model as PersonSchema;"));
    assert!(aliases.contains("pub use self::r#type::Model as TypeSchema;"));
    assert!(!aliases.contains("stale_table"));
    assert!(!aliases.contains("prelude"));
    assert!(aliases.contains("mod sea_orm_active_enums;"));
    assert!(
        aliases.contains("pub use self::sea_orm_active_enums::UserStatus as UserStatusSchema;")
    );
    assert!(aliases.contains("pub use self::sea_orm_active_enums::Role as RoleSchema;"));
    assert!(!aliases.contains("::*"));
    assert!(!aliases.contains("InternalSchema"));
    // Regeneration reads our private declarations and produces the same facade.
    SchemaAliases::generate(root).unwrap();
    assert_eq!(fs::read_to_string(root.join("mod.rs")).unwrap(), aliases);
}

#[test]
fn enum_and_model_alias_collisions_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let original = "pub mod statuses; pub mod sea_orm_active_enums;";
    fs::write(root.join("mod.rs"), original).unwrap();
    fs::write(root.join("statuses.rs"), "pub struct Model;").unwrap();
    fs::write(
        root.join("sea_orm_active_enums.rs"),
        "pub enum Status { Active }",
    )
    .unwrap();
    let error = SchemaAliases::generate(root).unwrap_err();
    assert!(error.to_string().contains("StatusSchema is ambiguous"));
    assert_eq!(fs::read_to_string(root.join("mod.rs")).unwrap(), original);
}

#[test]
fn conflicting_names_fail_without_replacing_existing_aliases() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let original = "pub mod user; pub mod users;";
    fs::write(root.join("mod.rs"), original).unwrap();
    for name in ["user", "users"] {
        fs::write(root.join(format!("{name}.rs")), "pub struct Model;").unwrap();
    }
    let error = SchemaAliases::generate(root).unwrap_err();
    assert!(error.to_string().contains("UserSchema is ambiguous"));
    assert_eq!(fs::read_to_string(root.join("mod.rs")).unwrap(), original);
}
