use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
// use crate::visitor::logic::api::visitor_eto::VisitorEto;
use crate::visitor::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use anyhow::{Result};
// use suborbital::runnable::HostErr;

pub struct VisitorService;

impl Service<Vec<u8>, VisitorSearchCriteria, i64> for VisitorService {
    fn get_by_id(id: i64) -> Result<Option<Vec<u8>>> {

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let visitor_query_result = db::select("SelectVisitor", query_args);
        return match visitor_query_result {
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

    fn search(search_criteria: VisitorSearchCriteria) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn delete(id: i64) -> Result<i64> {
        Ok(id)
    }

    fn create(eto: Vec<u8>) -> Result<Vec<u8>> {
        Ok(eto)
    }
}