use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
use crate::visitor::logic::api::visitor_eto::VisitorEto;
use crate::visitor::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use anyhow::Result;

pub struct VisitorService;

impl Service<VisitorEto, VisitorSearchCriteria, i64> for VisitorService {
    fn get_by_id(id: i64) -> Result<Option<VisitorEto>> {

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let visitor = db::select("SelectVisitor", query_args);

        return match visitor {
            Ok(res) => {
                let visitor_result : Result<VisitorEto, serde_json::Error> =
                    serde_json::from_slice(&res);
                match visitor_result {
                    Ok(visitor) => Ok(Some(visitor)),
                    Err(_) => Ok(None)
                }
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        };
    }

    fn search(search_criteria: VisitorSearchCriteria) -> Result<SearchResult<VisitorEto>> {
        Ok(SearchResult {
            content: vec![],
            pageable: Pageable {
                page_size: 0,
                page_number: 0,
                sort: None
            },
            total_elements: 0
        })
    }

    fn delete(id: i64) -> Result<i64> {
        Ok(id)
    }

    fn create(eto: VisitorEto) -> Result<VisitorEto> {
        Ok(eto)
    }
}