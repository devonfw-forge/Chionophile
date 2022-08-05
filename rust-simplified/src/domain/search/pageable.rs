use serde::{Serialize, Deserialize};

use super::sort::Sort;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct Pageable {
    pub page_size: i32,
    pub page_number: i32,
    pub sort: Option<Vec<Sort>>
}

impl Pageable {
    pub fn from<T: Clone>(
        &self,
        vector: Vec<T>
    ) -> Vec<T> {
        let mut paged_results = Vec::new();
        if !vector.is_empty() {
            let starting_index = (self.page_size * self.page_number) as usize;
            if starting_index < vector.len() {
                if starting_index + self.page_size as usize <= vector.len() {
                    paged_results =
                        vector[starting_index..starting_index + self.page_size as usize]
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
