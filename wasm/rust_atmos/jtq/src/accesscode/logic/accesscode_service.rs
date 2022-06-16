use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
// use crate::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use crate::accesscode::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use anyhow::{Result};
// use suborbital::runnable::HostErr;

pub struct AccessCodeService;

impl Service<Vec<u8>, AccessCodeSearchCriteria, i64> for AccessCodeService {
    fn get_by_id(id: i64) -> Result<Option<Vec<u8>>> {

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let accesscode_query_result = db::select("SelectAccessCode", query_args);
        return match accesscode_query_result {
            Ok(query_res) => {
                if query_res.len() > 2 {
                    Ok(Some(query_res[1..query_res.len() - 1].to_vec()))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        }
/*        return match visitor_query_result {
            Ok(res) => {
                println!("{:#?}", res);
                let visitor_deserialized : Result<VisitorEto, Box<bincode::ErrorKind>> =
                    bincode::deserialize(&res[1..res.len() - 1]);
                match visitor_deserialized {
                    Ok(visitor) => {
                        println!("{:#?}", visitor);
                        Ok(Some(visitor))
                    }
                    Err(_) => {
                        println!("Error deseralizing");
                        Err(anyhow::Error::msg("Error deseralizing"))
                    }
                }
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        };
*/    }

    fn search(search_criteria: AccessCodeSearchCriteria) -> Result<SearchResult<Vec<u8>>> {
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

    fn create(eto: Vec<u8>) -> Result<Vec<u8>> {
        Ok(eto)
    }
}