use serde::Serialize;

use super::pageable::Pageable;

/*
    This struct is made to answer all the find requests. It contains the pageable
    included in that request, a counter of elements and the search results.
*/
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct SearchResult<T> {
    pub content: Vec<T>,
    pub pageable: Pageable,
    pub total_elements: i32
}

impl<T> SearchResult<T> {

    pub fn new(
        content: Vec<T>,
        pageable: Pageable,
        total_elements: i32
    ) -> SearchResult<T> {

        SearchResult::<T> {
            content,
            pageable,
            total_elements
        }
    }

}