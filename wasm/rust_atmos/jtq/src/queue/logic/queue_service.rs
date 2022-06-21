use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
use crate::queue::dataaccess::api::queue::QueueEntity;
use crate::queue::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::queue::logic::api::queue_eto::QueueEto;

use chrono::Utc;
use serde_json::Value;
use suborbital::db;
use suborbital::db::query;
use anyhow::{Result, anyhow};

pub struct QueueService;

impl Service<QueueSearchCriteria, i64> for QueueService {
    fn get_by_id(id: i64) -> Result<Option<Vec<u8>>> {

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let queue_query_result = db::select("SelectQueue", query_args);

        if let Err(query_error) = queue_query_result {
            return Err(anyhow::Error::msg(query_error.message))
        }

        let query_str = String::from_utf8(queue_query_result.unwrap_or_default())?;
        let entities:Vec<QueueEntity> = serde_json::from_str(query_str.as_str())?;
        
        if entities.len() > 0 {
            let eto: QueueEto = entities.get(0).unwrap().clone().into();
            let res = serde_json::to_string(&eto)?;
            Ok(Some(res.as_bytes().to_vec()))
        } else {
            Ok(None)
        }
    }

    fn search(criteria: QueueSearchCriteria) -> Result<Vec<u8>> {
        let mut query_args: Vec<query::QueryArg> = Vec::new();

        let results = if criteria.active.is_some() {
            query_args.push(
                query::QueryArg::new(
                    "active",
                    &criteria.active.unwrap_or_default().to_string()
                )
            );
            db::select("SearchQueueActive", query_args)
        } else {
            db::select("SearchQueue", query_args)
        };

        if let Err(e) = results {
            println!("{}", e.message);
            return Err(anyhow!("Error searching for queues"));
        }
        let entities_as_string = String::from_utf8(results.unwrap_or_default())?;
        let entities: Vec<QueueEntity> = match serde_json::from_str(&entities_as_string) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                Vec::new()
            }
        };

        let total_elements = entities.len();

        let paged_entities = Pageable::from(&criteria.pageable, entities);
        let content: Vec<QueueEto> = paged_entities.iter()
            .map(|entity| entity.clone().into())
            .collect();

        let search_res = SearchResult::new(content, criteria.pageable.clone(), total_elements as i32);
        let res = serde_json::to_string(&search_res)?;

        Ok(res.as_bytes().to_vec())
    }

    fn delete(id: i64) -> Result<i64> {
        let mut query_args1: Vec<query::QueryArg> = Vec::new();
        let mut query_args2: Vec<query::QueryArg> = Vec::new();
        query_args1.push(query::QueryArg::new("id", &id.to_string()));
        query_args2.push(query::QueryArg::new("id", &id.to_string()));

        let _ac_query_result = db::delete("DeleteACQueue", query_args1);
        let queue_query_result = db::delete("DeleteQueue", query_args2);
        
        if let Ok(query_res) = queue_query_result {
            let query_res_json: Value = serde_json::from_str(&String::from_utf8(query_res).unwrap())?;
            if query_res_json["rowsAffected"].as_i64().unwrap() < 1 {
                return Err(anyhow!("Not found"));
            }
            Ok(id)
        } else {
            println!("Error deleting from db {} ", queue_query_result.err().unwrap().message);
            Err(anyhow!("Error deleting from database"))
        }
    }

    fn create(eto: Vec<u8>) -> Result<Vec<u8>> {
        println!("Create visitor");
        let queue_string = String::from_utf8(eto);

        let queue: QueueEto = serde_json::from_str(&queue_string.unwrap()).unwrap();
        
        let mut eto_res = queue.clone();
        println!("Insertig queryargs");
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(
            query::QueryArg::new(
                "modificationcounter",
                &queue.modification_counter.unwrap_or(1).to_string()
            )
        );
        query_args.push(
            query::QueryArg::new(
                "name",
                &queue.name.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "logo",
                &queue.logo.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "currentnumber",
                &queue.current_number.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "attentiontime",
                &queue.attention_time.unwrap_or(Utc::now().naive_utc().to_string()).to_string()
            )
        );
        query_args.push(
            query::QueryArg::new(
                "minattentiontime",
                &queue.min_attention_time.to_string()
            )
        );
        query_args.push(
            query::QueryArg::new(
                "active",
                &queue.active.to_string()
            )
        );

        let queue_query_result = db::insert("CreateQueue", query_args);

        if let Ok(_) = queue_query_result {
            let queue_id = db::select("SelectLastIdQueue", vec![]).unwrap_or_default();
            let id_vec_string = String::from_utf8(queue_id)?;
            let id_json: Value = serde_json::from_str(&id_vec_string)?;
            let id: i64 = id_json[0]["currval"].to_string().parse().unwrap();
            eto_res.id = Option::from(id);
            let res = serde_json::to_string(&eto_res)?;
            Ok(res.as_bytes().to_vec())
        } else {
            Err(anyhow!("Could not insert Queue"))
        }
    }
}