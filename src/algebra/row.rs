use std::collections::HashMap;

pub enum Row<K: Eq + Clone> {
    SimpleRow(K, HashMap<String, String>),
    LeftJoinRow {
        key: K,
        left: Box<Row<K>>,
        right: Option<Box<Row<K>>>,
    },
}

pub enum RowFrame<K: Eq + Clone, A> {
    SimpleRow(HashMap<String, String>),
    LeftJoinRow { key: K, left: A, right: A },
}
