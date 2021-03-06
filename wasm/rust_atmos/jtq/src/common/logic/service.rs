use crate::common::search::criteria::Criteria;
use anyhow::Result;

pub trait Service<SC, ID>
    where
        SC: Criteria
{
    fn get_by_id(id: ID) -> Result<Option<Vec<u8>>>;
    fn search(search_criteria: SC) -> Result<Vec<u8>>;
    fn delete(id: ID) -> Result<ID>;
    fn create(eto: Vec<u8>) -> Result<Vec<u8>>;
}
