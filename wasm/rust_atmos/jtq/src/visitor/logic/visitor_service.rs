use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
use crate::visitor::logic::api::visitor_eto::VisitorEto;
use crate::visitor::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use validator::Validate;
use anyhow::{anyhow, Result};
use serde_json::Value;
use crate::visitor::dataaccess::api::visitor::VisitorEntity;

pub struct VisitorService;

impl Service<VisitorSearchCriteria, i64> for VisitorService {
    fn get_by_id(id: i64) -> Result<Option<Vec<u8>>> {
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));
        let visitor_query_result: Result<Vec<u8>, _> = db::select("SelectVisitor", query_args);

        if let Err(query_error) = visitor_query_result {
           return Err(anyhow::Error::msg(query_error.message));
        }
        let query_as_string = String::from_utf8(visitor_query_result.unwrap_or_default())?;
        let entities : Vec<VisitorEntity> = serde_json::from_str(query_as_string.as_str())?;

        if entities.len() > 0 {
            let eto: VisitorEto = entities.get(0).unwrap().clone().into();
            let res = serde_json::to_string(&eto)?;
            Ok(Some(res.as_bytes().to_vec()))
        } else {
            Ok(None)
        }
    }

    fn search(criteria: VisitorSearchCriteria) -> Result<Vec<u8>> {
        let mut query_args: Vec<query::QueryArg> = Vec::new();

        let results = if criteria.username.is_some() {
            query_args.push(
                query::QueryArg::new(
                    "username",
                    &criteria.username.unwrap_or_default()
                )
            );
            query_args.push(
                query::QueryArg::new(
                    "password",
                    &criteria.password.unwrap_or_default()
                )
            );

            db::select("SearchVisitorWithParams", query_args)
        } else {
            db::select("SearchVisitor", query_args)
        };

        if let Err(e) = results {
            println!("{}", e.message);
            return Err(anyhow!("Error searching for visitors"));
        }
        let entities_as_string = String::from_utf8(results.unwrap_or_default())?;
        let entities: Vec<VisitorEntity> = serde_json::from_str(&entities_as_string)?;
        let total_elements = entities.len();

        let paged_entities = Pageable::from(&criteria.pageable, entities);
        let content: Vec<VisitorEto> = paged_entities.iter()
            .map(|entity| entity.clone().into())
            .collect();

        let search_res = SearchResult::new(content, criteria.pageable.clone(), total_elements as i32);
        let res = serde_json::to_string(&search_res)?;

        Ok(res.as_bytes().to_vec())
    }

    fn delete(id: i64) -> Result<i64> {
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let _accesscode_query_result =
            db::delete("DeleteAccessCodeByIdVisitor", query_args.clone());
        let visitor_query_result =
            db::delete("DeleteVisitor", query_args);
        if let Ok(query_res) = visitor_query_result {
            let query_res_json: Value = serde_json::from_str(&String::from_utf8(query_res).unwrap())?;
            if query_res_json["rowsAffected"].as_i64().unwrap() < 1 {
                return Err(anyhow!("Not found"));
            }
            Ok(id)
        } else {
            println!("Error deleting from db {} ", visitor_query_result.err().unwrap().message);
            Err(anyhow!("Error deleting from database"))
        }

    }

    fn create(eto: Vec<u8>) -> Result<Vec<u8>> {
        println!("Create visitor");
        let visitor_string = String::from_utf8(eto);
        let visitor: VisitorEto = serde_json::from_str(&visitor_string.unwrap())?;
        if let Err(errors) = visitor.validate() {
            return Err(anyhow::Error::from(errors));
        }
        let mut eto_res = visitor.clone();
        println!("Insertig queryargs");
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(
            query::QueryArg::new(
                "acceptedcommercial",
                &visitor.accepted_commercial.unwrap_or(false).to_string()
            )
        );
        query_args.push(
            query::QueryArg::new(
                "acceptedterms",
                &visitor.accepted_terms.to_string()
            )
        );
        query_args.push(
            query::QueryArg::new(
                "modificationcounter",
                &visitor.modification_counter.unwrap_or(1).to_string()
            )
        );
        query_args.push(
            query::QueryArg::new(
                "name",
                &visitor.name.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "password",
                &visitor.password.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "phonenumber",
                &visitor.phone_number.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "username",
                &visitor.username.unwrap_or("".to_string())
            )
        );
        query_args.push(
            query::QueryArg::new(
                "usertype",
                &visitor.user_type.unwrap_or(false).to_string()
            )
        );

        let visitor_query_result = db::insert("CreateVisitor", query_args);

        if let Ok(_) = visitor_query_result {
            let visitor_id = db::select("SelectLastIdVisitor", vec![]).unwrap_or_default();
            let id_vec_string = String::from_utf8(visitor_id)?;
            let id_json: Value = serde_json::from_str(&id_vec_string)?;
            let id: i64 = id_json[0]["currval"].to_string().parse().unwrap();
            eto_res.id = Option::from(id);
            let res = serde_json::to_string(&eto_res)?;
            Ok(res.as_bytes().to_vec())
        } else {
            Err(anyhow!("Could not insert Visitor"))
        }
    }
}