//! Helpers for tests that need a facade [`Context`].

use std::{ops::Deref, sync::Arc};

use mosaicod_db as db;
use mosaicod_query as query;
use mosaicod_store as store;

use crate::Context;

/// A [`Context`] backed by a test database and a temporary store.
///
/// The store directory is deleted when this value is dropped, so keep it
/// alive for the whole test.
pub struct TestContext {
    context: Context,
    _store: store::testing::Store,
}

impl Deref for TestContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

/// Creates a [`TestContext`] on the given test database pool and a new
/// temporary store.
pub fn test_context(pool: sqlx::Pool<db::DatabaseType>) -> TestContext {
    let database = db::testing::Database::new(pool);
    let store = store::testing::Store::new_random_on_tmp().unwrap();
    let ts_gw = Arc::new(query::TimeseriesEngine::try_new((*store).clone(), 0).unwrap());

    TestContext {
        context: Context::new((*store).clone(), (*database).clone(), ts_gw),
        _store: store,
    }
}
