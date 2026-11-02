use sea_orm_migration::{
    prelude::*,
    sea_query::{ColumnType, TableAlterStatement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::add_capabilities()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(Self::drop_capabilities()).await
    }
}

impl Migration {
    fn add_capabilities() -> TableAlterStatement {
        Table::alter()
            .table(ProviderModels::Table)
            .add_column(
                ColumnDef::new(ProviderModels::ReasoningEffortValues)
                    .array(ColumnType::Text)
                    .check(Expr::cust(
                        "array_position(reasoning_effort_values, NULL) IS NULL",
                    )),
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
            .drop_column(ProviderModels::ReasoningEffortValues)
            .to_owned()
    }
}

#[derive(DeriveIden)]
enum ProviderModels {
    Table,
    ReasoningEffortValues,
    SupportsReasoningBudget,
    SupportsAdaptiveReasoning,
    MinimumReasoningBudgetTokens,
    MaximumReasoningBudgetTokens,
}
