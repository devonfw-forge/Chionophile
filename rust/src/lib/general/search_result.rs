use serde::Serialize;
use crate::lib::general::pageable::Pageable;

#[derive(Serialize)]
pub struct SearchResult<T> {
    pub content: Vec<T>,
    pub pageable: Pageable,
    pub totalElements: i32
}

impl<T> SearchResult<T> {

    pub fn new(
        content: Vec<T>,
        pageable: Pageable,
        totalElements: i32
    ) -> SearchResult<T> {

        SearchResult::<T> {
            content,
            pageable,
            totalElements
        }
    }
 
}