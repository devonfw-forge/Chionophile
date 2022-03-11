use serde::{Serialize, Deserialize};
use crate::lib::general::sort::Sort;

#[derive(Deserialize, Serialize, Clone)]
pub struct Pageable {
    pub pageSize: i32,
    pub pageNumber: i32,
    pub sort: Option<Vec<Sort>>
}

impl Pageable {
    pub fn from<T: Clone>(
        &self,
        vector: Vec<T>
    ) -> Vec<T> {
        let mut paged_results = Vec::new();
        if !vector.is_empty() {
            let starting_index = (self.pageSize * self.pageNumber) as usize;
            if starting_index < vector.len() {
                if starting_index + self.pageSize as usize <= vector.len() {
                    paged_results =
                        vector[starting_index..starting_index + self.pageSize as usize]
                            .to_vec();
                } else {
                    paged_results =
                        vector[starting_index..]
                            .to_vec();
                }
            }
        }
        paged_results
    }

}
