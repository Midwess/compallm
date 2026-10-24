use axum::extract::FromRef;
use loco_rs::app::AppContext;
use sea_orm::DatabaseConnection;
use std::ops::Deref;

/// A cloneable handle to the SeaORM pool created and managed by Loco.
#[derive(Clone)]
pub struct Connection(DatabaseConnection);

impl Connection {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self(connection)
    }

    pub fn from_context(ctx: &AppContext) -> Self {
        Self::new(ctx.db.clone())
    }

    pub fn sea_orm(&self) -> &DatabaseConnection {
        &self.0
    }

    pub fn into_sea_orm(self) -> DatabaseConnection {
        self.0
    }
}

impl FromRef<AppContext> for Connection {
    fn from_ref(ctx: &AppContext) -> Self {
        Self::from_context(ctx)
    }
}

impl Deref for Connection {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        self.sea_orm()
    }
}

impl AsRef<DatabaseConnection> for Connection {
    fn as_ref(&self) -> &DatabaseConnection {
        self.sea_orm()
    }
}

impl From<DatabaseConnection> for Connection {
    fn from(connection: DatabaseConnection) -> Self {
        Self::new(connection)
    }
}
