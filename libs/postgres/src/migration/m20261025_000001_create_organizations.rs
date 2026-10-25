use sea_orm_migration::{
    prelude::*,
    sea_query::{
        IndexCreateStatement, TableCreateStatement,
        extension::postgres::{Type, TypeCreateStatement},
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(Self::organization_status_type())
            .await?;
        manager.create_type(Self::organization_role_type()).await?;
        manager.create_table(Self::organizations_table()).await?;
        manager
            .create_table(Self::organization_members_table())
            .await?;
        manager.create_index(Self::creator_index()).await?;
        manager
            .create_index(Self::member_organization_index())
            .await?;
        manager.create_index(Self::member_user_index()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Organizations::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OrganizationRole::Enum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OrganizationStatus::Enum).to_owned())
            .await
    }
}

impl Migration {
    fn organization_status_type() -> TypeCreateStatement {
        Type::create()
            .as_enum(OrganizationStatus::Enum)
            .values([OrganizationStatus::Active, OrganizationStatus::Disabled])
            .to_owned()
    }

    fn organization_role_type() -> TypeCreateStatement {
        Type::create()
            .as_enum(OrganizationRole::Enum)
            .values([
                OrganizationRole::Owner,
                OrganizationRole::Admin,
                OrganizationRole::Member,
            ])
            .to_owned()
    }

    fn organizations_table() -> TableCreateStatement {
        Table::create()
            .table(Organizations::Table)
            // Hero generates the ID in the application; no database sequence.
            .col(
                ColumnDef::new(Organizations::Id)
                    .big_integer()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Organizations::Name).text().not_null())
            .col(
                ColumnDef::new(Organizations::Status)
                    .custom(OrganizationStatus::Enum)
                    .not_null()
                    .default("active"),
            )
            .col(
                ColumnDef::new(Organizations::CreatedBy)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Organizations::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Organizations::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .check(Expr::col(Organizations::Id).gt(0))
            .check(Expr::cust("length(btrim(name)) > 0"))
            .foreign_key(
                ForeignKey::create()
                    .name("organizations_created_by_fkey")
                    .from(Organizations::Table, Organizations::CreatedBy)
                    .to(Users::Table, Users::Id)
                    // Preserve the identity of the creator for audit purposes.
                    .on_delete(ForeignKeyAction::Restrict),
            )
            .to_owned()
    }

    fn organization_members_table() -> TableCreateStatement {
        Table::create()
            .table(OrganizationMembers::Table)
            .col(
                ColumnDef::new(OrganizationMembers::OrganizationId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(OrganizationMembers::UserId)
                    .big_integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(OrganizationMembers::Role)
                    .custom(OrganizationRole::Enum)
                    .not_null(),
            )
            .col(
                ColumnDef::new(OrganizationMembers::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .primary_key(
                Index::create()
                    .name("organization_members_pkey")
                    .col(OrganizationMembers::OrganizationId)
                    .col(OrganizationMembers::UserId),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("organization_members_organization_id_fkey")
                    .from(
                        OrganizationMembers::Table,
                        OrganizationMembers::OrganizationId,
                    )
                    .to(Organizations::Table, Organizations::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("organization_members_user_id_fkey")
                    .from(OrganizationMembers::Table, OrganizationMembers::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn creator_index() -> IndexCreateStatement {
        Index::create()
            .name("organizations_created_by_idx")
            .table(Organizations::Table)
            .col(Organizations::CreatedBy)
            .to_owned()
    }

    fn member_user_index() -> IndexCreateStatement {
        // Reverse lookup for listing the organizations to which a user belongs.
        Index::create()
            .name("organization_members_user_id_idx")
            .table(OrganizationMembers::Table)
            .col(OrganizationMembers::UserId)
            .to_owned()
    }

    fn member_organization_index() -> IndexCreateStatement {
        Index::create()
            .name("organization_members_organization_id_idx")
            .table(OrganizationMembers::Table)
            .col(OrganizationMembers::OrganizationId)
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum OrganizationStatus {
    #[sea_orm(iden = "organization_status")]
    Enum,
    Active,
    Disabled,
}

#[derive(DeriveIden)]
enum OrganizationRole {
    #[sea_orm(iden = "organization_role")]
    Enum,
    Owner,
    Admin,
    Member,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
    Name,
    Status,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
    Role,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn organization_identity_uses_bigint_and_preserves_creator() {
        let sql = Migration::organizations_table().to_string(PostgresQueryBuilder);
        assert!(sql.contains(r#""id" bigint NOT NULL PRIMARY KEY"#));
        assert!(!sql.to_lowercase().contains("serial"));
        assert!(sql.contains(r#""status" organization_status NOT NULL DEFAULT 'active'"#));
        assert!(sql.contains(
            r#"FOREIGN KEY ("created_by") REFERENCES "users" ("id") ON DELETE RESTRICT"#
        ));
        assert!(sql.contains("length(btrim(name)) > 0"));
        assert!(sql.contains(
            r#""created_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP"#
        ));
        assert!(sql.contains(
            r#""updated_at" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP"#
        ));
    }

    #[test]
    fn membership_has_compound_primary_key_and_individual_lookup_indexes() {
        let sql = Migration::organization_members_table().to_string(PostgresQueryBuilder);
        assert!(sql.contains(r#"PRIMARY KEY ("organization_id", "user_id")"#));
        assert!(sql.contains(r#""role" organization_role NOT NULL"#));
        assert!(sql.contains(
            r#"FOREIGN KEY ("organization_id") REFERENCES "organizations" ("id") ON DELETE CASCADE"#
        ));
        assert!(
            sql.contains(r#"FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE"#)
        );
        assert_eq!(
            Migration::member_organization_index().to_string(PostgresQueryBuilder),
            r#"CREATE INDEX "organization_members_organization_id_idx" ON "organization_members" ("organization_id")"#
        );
        assert_eq!(
            Migration::member_user_index().to_string(PostgresQueryBuilder),
            r#"CREATE INDEX "organization_members_user_id_idx" ON "organization_members" ("user_id")"#
        );
    }

    #[test]
    fn status_and_role_are_native_postgres_enums() {
        assert_eq!(
            Migration::organization_status_type().to_string(PostgresQueryBuilder),
            r#"CREATE TYPE "organization_status" AS ENUM ('active', 'disabled')"#
        );
        assert_eq!(
            Migration::organization_role_type().to_string(PostgresQueryBuilder),
            r#"CREATE TYPE "organization_role" AS ENUM ('owner', 'admin', 'member')"#
        );
    }
}
