//! Application dependency construction, using the database configured by Loco.

use compallm_core::persistent::DbContext;
use loco_rs::app::AppContext;
use std::sync::Arc;

#[derive(Clone)]
pub struct Dependencies {
    pub db_context: DbContext,
}

impl Dependencies {
    pub fn new(ctx: &AppContext) -> Self {
        Self {
            db_context: DbContext::new(Arc::new(ctx.db.clone())),
        }
    }
}
