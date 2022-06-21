use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
// use crate::accesscode::dataaccess::api::accesscode::AccessCodeEntity;
// use crate::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use crate::accesscode::logic::api::accesscode_cto::{AccessCodeCto, AccessCodeQueryResult};
use crate::accesscode::logic::api::accesscode_insert::AccessCodeInsert;
use crate::accesscode::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use anyhow::{anyhow, Result};

pub struct AccessCodeService;

impl Service<AccessCodeSearchCriteria, i64> for AccessCodeService {
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
    }

    fn create(input: Vec<u8>) -> Result<Vec<u8>> {
        let in_string = String::from_utf8(input).unwrap();
        let accesscode_insert: AccessCodeInsert = serde_json::from_str(&in_string)?;

        let mut query_args: Vec<query::QueryArg> = Vec::new();

        query_args.push(query::QueryArg::new("idvisitor", &accesscode_insert.visitor_id.to_string()));
        query_args.push(query::QueryArg::new("idqueue", &accesscode_insert.queue_id.to_string()));

        let accesscode_query_result = db::insert("InsertAccessCode", query_args);
        return match accesscode_query_result {
            Ok(query_res) => {
                Ok(query_res[1..query_res.len() - 1].to_vec())
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        }
    }

    fn search(criteria: AccessCodeSearchCriteria) -> Result<Vec<u8>> {
        let query_args: Vec<query::QueryArg> = Vec::new();

        let results = db::select("SearchAccessCode", query_args);
        
        if let Err(e) = results {
            println!("{}", e.message);
            return Err(anyhow!("Error searching for visitors"));
        }

        let entities_as_string = String::from_utf8(results.unwrap_or_default())?;
        println!("Despues de to string {}", entities_as_string);
        let entities: Vec<AccessCodeQueryResult> = match serde_json::from_str(&entities_as_string) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                Vec::new()
            }
        };
        println!("Despues de to string {:?}", entities_as_string);
        let total_elements = entities.len();

        let paged_entities = Pageable::from(&criteria.pageable, entities);
        let content: Vec<AccessCodeCto> = paged_entities.iter()
            .map(|entity| entity.clone().into())
            .collect();

        let search_res = SearchResult::new(content, criteria.pageable.clone(), total_elements as i32);
        let res = serde_json::to_string(&search_res)?;

        Ok(res.as_bytes().to_vec())
    }

    fn delete(id: i64) -> Result<i64> {
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let accesscode_query_result = db::delete("DeleteAccessCode", query_args);
        return match accesscode_query_result {
            Ok(_) => {
                Ok(id)
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        }
    }
}