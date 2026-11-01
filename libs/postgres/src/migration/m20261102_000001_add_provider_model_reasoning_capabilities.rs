use sea_orm_migration::{
    prelude::*,
    sea_query::{
        ColumnType, TableAlterStatement,
        extension::postgres::{Type, TypeCreateStatement},
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Self::reasoning_support_type()).await?;
        manager.alter_table(Self::add_capabilities()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::drop_capabilities()).await?;
        manager
            .drop_type(Type::drop().name(ReasoningSupport::Enum).to_owned())
            .await
    }
}

impl Migration {
    fn reasoning_support_type() -> TypeCreateStatement {
        Type::create()
            .as_enum(ReasoningSupport::Enum)
            .values([
                ReasoningSupport::Unknown,
                ReasoningSupport::Unsupported,
                ReasoningSupport::Supported,
            ])
            .to_owned()
    }

    fn add_capabilities() -> TableAlterStatement {
        Table::alter()
            .table(ProviderModels::Table)
            .add_column(
                ColumnDef::new(ProviderModels::ReasoningSupport)
                    .custom(ReasoningSupport::Enum)
                    .not_null()
                    .default("unknown"),
            )
            .add_column(
                ColumnDef::new(ProviderModels::ReasoningEffortValues)
                    .array(ColumnType::SmallInteger)
                    .not_null()
                    .default(Expr::cust("'{}'::smallint[]"))
                    .check(Expr::cust(
                        "array_position(reasoning_effort_values, NULL) IS NULL \
                         AND 0 < ALL(reasoning_effort_values) \
                         AND 255 >= ALL(reasoning_effort_values)",
                    )),
            )
            .add_column(
                ColumnDef::new(ProviderModels::SupportsReasoningEffort)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .add_column(
                ColumnDef::new(ProviderModels::SupportsReasoningBudget)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .add_column(
                ColumnDef::new(ProviderModels::SupportsAdaptiveReasoning)
                    .boolean()
                    .not_null()
                    .default(false),
            )
            .add_column(
                ColumnDef::new(ProviderModels::MinimumReasoningBudgetTokens)
                    .integer()
                    .check(
                        Expr::col(ProviderModels::MinimumReasoningBudgetTokens)
                            .is_null()
                            .or(Expr::col(ProviderModels::MinimumReasoningBudgetTokens).gt(0)),
                    ),
            )
            .add_column(
                ColumnDef::new(ProviderModels::MaximumReasoningBudgetTokens)
                    .integer()
                    .check(
                        Expr::col(ProviderModels::MaximumReasoningBudgetTokens)
                            .is_null()
                            .or(Expr::col(ProviderModels::MaximumReasoningBudgetTokens)
                                .gt(0)
                                .and(
                                    Expr::col(ProviderModels::MinimumReasoningBudgetTokens)
                                        .is_null()
                                        .or(Expr::col(
                                            ProviderModels::MaximumReasoningBudgetTokens,
                                        )
                                        .gte(Expr::col(
                                            ProviderModels::MinimumReasoningBudgetTokens,
                                        ))),
                                )),
                    ),
            )
            .to_owned()
    }

    fn drop_capabilities() -> TableAlterStatement {
        Table::alter()
            .table(ProviderModels::Table)
            .drop_column(ProviderModels::MaximumReasoningBudgetTokens)
            .drop_column(ProviderModels::MinimumReasoningBudgetTokens)
            .drop_column(ProviderModels::SupportsAdaptiveReasoning)
            .drop_column(ProviderModels::SupportsReasoningBudget)
            .drop_column(ProviderModels::SupportsReasoningEffort)
            .drop_column(ProviderModels::ReasoningEffortValues)
            .drop_column(ProviderModels::ReasoningSupport)
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    ReasoningSupport,
    ReasoningEffortValues,
    SupportsReasoningEffort,
    SupportsReasoningBudget,
    SupportsAdaptiveReasoning,
    MinimumReasoningBudgetTokens,
    MaximumReasoningBudgetTokens,
}

#[derive(DeriveIden)]
enum ReasoningSupport {
    #[sea_orm(iden = "reasoning_support")]
    Enum,
    Unknown,
    Unsupported,
    Supported,
}
