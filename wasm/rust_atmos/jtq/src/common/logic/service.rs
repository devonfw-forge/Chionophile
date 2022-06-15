use crate::common::logic::entity_eto::EntityETO;
use crate::common::search::criteria::Criteria;
use crate::common::search::search_result::SearchResult;
use anyhow::Result;

pub trait Service<ETO, SC, ID>
    where
        ETO: EntityETO,
        SC: Criteria
{
    fn get_by_id(id: ID) -> Result<Option<ETO>>;
    fn search(search_criteria: SC) -> Result<SearchResult<ETO>>;
    fn delete(id: ID) -> Result<ID>;
    fn create(eto: ETO) -> Result<ETO>;
}