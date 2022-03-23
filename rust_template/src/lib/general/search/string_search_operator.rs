use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum StringSearchOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Like,
    NotLike
}