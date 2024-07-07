/// # row module
///
/// this module provides the data structures for manipulating
/// rows of data with a common key type.
///
/// # Notes
///
/// 2024-07-05: because enum variants are not "types" we cannot
/// make them generic, and that prevents us from following the
/// pattern of a generic tuple type or GADT.
///
/// 2024-07-07: though it adds a runtime cost, i'm going to use
/// trait objects here _at the data row level_ to stay with the
/// design, where i'm trading off code compexlity for performance.
/// that's ok for me, as i'm building this library to support
/// code that will execute offline or as part of background tasks.
///
pub mod left_join_row;
pub mod rowlike;
pub mod source_row;
