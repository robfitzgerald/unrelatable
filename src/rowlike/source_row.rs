use std::{marker::PhantomData, rc::Rc};

use super::rowlike::RowLike;

/// a row of some source dataset that has been deserialized.
/// the "leaf" of a RowLike tree.
/// these row types are the only ones that hold the values;
/// everything else up the tree holds a reference.
/// phantom type introduced in order to comply with the RowLike
/// trait's design.
pub struct SourceRow<'a, K: Eq + Clone, T> {
    key: K,
    value: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, K: Eq + Clone, T> SourceRow<'a, K, T> {
    pub fn new(key: K, value: T) -> SourceRow<'a, K, T> {
        SourceRow {
            key,
            value,
            phantom: PhantomData,
        }
    }
}

impl<'a, K: Eq + Clone, T> RowLike<'a, K, T, ()> for SourceRow<'a, K, T> {
    fn get_key(&'a self) -> &K {
        &self.key
    }

    fn get_left(&'a self) -> Option<&'a T> {
        Some(&self.value)
    }

    fn get_right(&'a self) -> Option<&'a ()> {
        None
    }
}
