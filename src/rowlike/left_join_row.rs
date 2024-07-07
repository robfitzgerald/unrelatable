use std::borrow::Borrow;

use super::rowlike::RowLike;

/// two RowLikes which have been joined using a LEFT JOIN.
/// the LEFT row should always be present. the RIGHT row
/// is only present if there was a matching key.
pub struct LeftJoinRow<'a, K: Eq + Clone, L1, L2, R1, R2> {
    key: K,
    left: Box<&'a dyn RowLike<K, L1, L2>>,
    right: Option<Box<&'a dyn RowLike<K, R1, R2>>>,
}

impl<'a, K: Eq + Clone, L1, L2, R1, R2> LeftJoinRow<'a, K, L1, L2, R1, R2> {
    pub fn new(
        key: K,
        left: Box<&'a dyn RowLike<K, L1, L2>>,
        right: Option<Box<&'a dyn RowLike<K, R1, R2>>>,
    ) -> LeftJoinRow<'a, K, L1, L2, R1, R2> {
        LeftJoinRow { key, left, right }
    }
}

impl<'a, K: Eq + Clone, L1, L2, R1, R2>
    RowLike<K, &'a dyn RowLike<K, L1, L2>, &'a dyn RowLike<K, R1, R2>>
    for LeftJoinRow<'a, K, L1, L2, R1, R2>
{
    fn get_key(&self) -> &K {
        &self.key
    }

    fn get_left(&self) -> Option<&Box<&&'a dyn RowLike<K, L1, L2>>> {
        todo!()
    }

    fn get_right(&self) -> Option<&Box<&&'a dyn RowLike<K, R1, R2>>> {
        todo!()
    }

    // fn get_left(&self) -> Option<&Box<&dyn RowLike<K, L1, L2>>> {
    //     Some(self.left)
    // }

    // fn get_right(&self) -> Option<&Box<&dyn RowLike<K, R1, R2>>> {
    //     self.right.clone()
    // }
}

// impl LeftJoinRow {
//     pub fn new<K: Eq + Clone, L1, L2, R1, R2>(
//         left: Rc<dyn RowLike<K, L1, L2>>,
//     )
// }
