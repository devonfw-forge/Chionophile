use chrono::NaiveDateTime;
use crate::models::pageable::Pageable;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSearchCriteria {
    pub name : Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub pageable: Pageable,
}

impl QueueSearchCriteria {
   pub fn generate_test_struct(page_size: i32) -> Self {
       QueueSearchCriteria {
           name: None,
           logo: None,
           current_number: None,
           attention_time: None,
           min_attention_time: None,
           active: None,
           pageable: Pageable {
               page_size,
               page_number: 0,
               sort: Some(vec![])
           }
       }
   } 
}