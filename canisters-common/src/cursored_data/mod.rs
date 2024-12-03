pub mod posts;
pub mod ref_history;
pub mod token_roots;
pub mod transaction;
pub mod vote;

use std::{error::Error, future::Future, hash::Hash};

pub struct PageEntry<T> {
    pub data: Vec<T>,
    pub end: bool,
}

/// Globally Unique key for the given type
pub trait KeyedData {
    type Key: Eq + Hash + 'static;

    fn key(&self) -> Self::Key;
}
pub trait CursoredDataProvider: Send + Sync {
    type Data: KeyedData + Clone + 'static + Send + Sync;
    type Error: Error;

    fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> impl Future<Output = Result<PageEntry<Self::Data>, Self::Error>> + Send;
}
