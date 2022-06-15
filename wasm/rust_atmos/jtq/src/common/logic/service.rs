use crate::common::logic::entity_eto::EntityETO;
use crate::common::search::criteria::Criteria;
use crate::common::search::search_result::SearchResult;

pub trait Service<ETO, SC, ID>
    where
        ETO: EntityETO,
        SC: Criteria
{
    fn get_by_id(id: ID) -> Option<ETO>;
    fn search(search_criteria: SC) -> SearchResult<ETO>;
    fn delete(id: ID) -> ID;
    fn create(eto: ETO) -> ETO;
}