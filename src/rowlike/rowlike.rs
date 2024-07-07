/// RowLikes are objects in the system that have the properties
/// of a row in a relational algebra.
pub trait RowLike<K: Eq + Clone, L, R> {
    fn get_key(&self) -> &K;
    fn get_left(&self) -> Option<&Box<&L>>;
    fn get_right(&self) -> Option<&Box<&R>>;
}
