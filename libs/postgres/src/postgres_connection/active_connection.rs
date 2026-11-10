use sea_orm::{
    DbErr, ExecResult, QueryResult, RuntimeErr, Statement,
    sea_query::Values,
    sqlx::{self, Executor, PgPool, Postgres, pool::PoolConnection},
};
use sea_query_sqlx::SqlxValues;

use crate::rls::RlsSettings;

pub(super) struct ActiveConnection<'a> {
    conn: PoolConnection<Postgres>,
    policy_applied: bool,
    settings: &'a [(String, String)],
    original_role: String,
}

impl<'a> ActiveConnection<'a> {
    pub(super) async fn acquire(
        pool: &PgPool,
        policy: Option<&'a RlsSettings>,
    ) -> Result<Self, DbErr> {
        let conn = pool
            .acquire()
            .await
            .map_err(|error| DbErr::Conn(RuntimeErr::SqlxError(error.into())))?;
        let mut active = Self {
            conn,
            policy_applied: false,
            settings: policy.map_or(&[], RlsSettings::settings),
            original_role: String::new(),
        };
        if policy.is_some() {
            active.original_role =
                sqlx::query_scalar(sqlx::AssertSqlSafe("SELECT current_setting('role')"))
                    .fetch_one(&mut *active.conn)
                    .await
                    .map_err(query_error)?;
            active.policy_applied = true;
            sqlx::query(sqlx::AssertSqlSafe("SET ROLE compallm_restricted"))
                .execute(&mut *active.conn)
                .await
                .map_err(exec_error)?;
            let (keys, values): (Vec<_>, Vec<_>) = active
                .settings
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .unzip();
            sqlx::query(sqlx::AssertSqlSafe(
                "SELECT set_config(key, value, false) FROM unnest($1::text[], $2::text[]) AS settings(key, value)"
            )).bind(keys).bind(values).execute(&mut *active.conn).await.map_err(exec_error)?;
        }
        Ok(active)
    }

    pub(super) async fn release(mut self) -> Result<(), DbErr> {
        if self.policy_applied {
            let keys: Vec<_> = self.settings.iter().map(|(key, _)| key.as_str()).collect();
            sqlx::query(sqlx::AssertSqlSafe(
                "SELECT set_config(key, '', false) FROM unnest($1::text[]) AS settings(key)",
            ))
            .bind(keys)
            .execute(&mut *self.conn)
            .await
            .map_err(exec_error)?;
            sqlx::query(sqlx::AssertSqlSafe("SELECT set_config('role', $1, false)"))
                .bind(&self.original_role)
                .execute(&mut *self.conn)
                .await
                .map_err(exec_error)?;
            self.policy_applied = false;
        }
        Ok(())
    }

    pub(super) async fn execute(&mut self, statement: &Statement) -> Result<ExecResult, DbErr> {
        query(statement)
            .execute(&mut *self.conn)
            .await
            .map(Into::into)
            .map_err(exec_error)
    }

    pub(super) async fn execute_unprepared(&mut self, sql: &str) -> Result<ExecResult, DbErr> {
        self.conn
            .execute(sqlx::AssertSqlSafe(sql))
            .await
            .map(Into::into)
            .map_err(exec_error)
    }

    pub(super) async fn query_one(
        &mut self,
        statement: &Statement,
    ) -> Result<Option<QueryResult>, DbErr> {
        query(statement)
            .fetch_optional(&mut *self.conn)
            .await
            .map(|row| row.map(Into::into))
            .map_err(query_error)
    }

    pub(super) async fn query_all(
        &mut self,
        statement: &Statement,
    ) -> Result<Vec<QueryResult>, DbErr> {
        query(statement)
            .fetch_all(&mut *self.conn)
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
            .map_err(query_error)
    }
}

impl Drop for ActiveConnection<'_> {
    fn drop(&mut self) {
        if self.policy_applied {
            self.conn.close_on_drop();
        }
    }
}

fn query(statement: &Statement) -> sqlx::query::Query<'_, Postgres, SqlxValues> {
    sqlx::query_with(
        sqlx::AssertSqlSafe(statement.sql.as_str()),
        SqlxValues(statement.values.clone().unwrap_or(Values(Vec::new()))),
    )
}

fn exec_error(error: sqlx::Error) -> DbErr {
    DbErr::Exec(RuntimeErr::SqlxError(error.into()))
}

fn query_error(error: sqlx::Error) -> DbErr {
    DbErr::Query(RuntimeErr::SqlxError(error.into()))
}
