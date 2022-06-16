use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
use crate::visitor::logic::api::visitor_eto::VisitorEto;
use crate::visitor::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use validator::Validate;
use anyhow::{anyhow, Result};

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
    }

    fn search(search_criteria: VisitorSearchCriteria) -> Result<SearchResult<Vec<u8>>> {
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
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let _accesscode_query_result =
            db::delete("DeleteAccessCodeByIdVisitor", query_args.clone());
        let visitor_query_result =
            db::delete("DeleteVisitor", query_args);
        if let Ok(_) = visitor_query_result {
            Ok(id)
        } else {
            println!("Error deleting from db {} ", visitor_query_result.err().unwrap().message);
            Err(anyhow!("Error deleting from database"))
        }

    }

    fn create(eto: Vec<u8>) -> Result<Vec<u8>> {
        println!("Create visitor");
        let visitor_string: Result<String, _> = String::from_utf8(eto);
        if let Err(e) = visitor_string {
            println!("{}", e);
            return Err(anyhow!("Not a valid string apparently"));
        }
        println!("Visitor serialize");
        let visitor: VisitorEto = serde_json::from_str(&visitor_string.unwrap())?;
        if let Err(errors) = visitor.validate() {
        println!("Validation errors");
            return Err(anyhow::Error::from(errors));
        }
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
        if let Ok(visitor_bytes) = visitor_query_result {
            Ok(visitor_bytes)
        } else {
            Err(anyhow!("Could not insert Visitor"))
        }
    }
}