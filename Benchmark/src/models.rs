use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct VisitorEto {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub phoneNumber: Option<String>,
    pub password: Option<String>,
    pub acceptedCommercial: Option<bool>,
    pub acceptedTerms: bool,
    pub userType: Option<bool>,
}

impl VisitorEto {
    pub fn generate_test_visitor() -> VisitorEto {
        let mail = format!("{}@mail.com", Uuid::new_v4());
        VisitorEto {
            id: None,
            username: Some(mail),
            name: Some("Goose".to_string()),
            phoneNumber: Some("123456789".to_string()),
            password: Some("1234".to_string()),
            acceptedCommercial: Some(true),
            acceptedTerms: true,
            userType: Some(true)
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeEto {
    pub id: Option<i64>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub queue_id: i64,
    pub visitor_id: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodePostData {
    pub visitor_id: i64,
    pub queue_id: i64
}

impl AccessCodePostData {
    pub fn new(
        visitor_id: i64,
        queue_id: i64
    ) -> AccessCodePostData {
        AccessCodePostData {
            visitor_id,
            queue_id
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct VisitorSearchCriteria {
    pub pageable: Pageable,
    pub username: Option<String>,
    pub password: Option<String>
}

impl VisitorSearchCriteria {
    pub fn generate_test_search_criteria(
        page_size: i32,
        username: Option<String>,
        password: Option<String>
    ) -> VisitorSearchCriteria {
        VisitorSearchCriteria {
            pageable: Pageable {
                pageSize: page_size,
                pageNumber: 0,
                sort: Some(vec![])
            },
            username,
            password
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueEto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
    pub customers: i32,
}

impl QueueEto {
    pub fn generate_test_queue() -> QueueEto {
        QueueEto {
            id: None,
            name: Some("TestQueue".to_string()),
            logo: Some("TestLogo".to_string()),
            current_number: Some("Q000".to_string()),
            attention_time: None,
            min_attention_time: Utc::now().naive_utc(),
            active: true,
            customers: 0
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Pageable {
    pub pageSize: i32,
    pub pageNumber: i32,
    pub sort: Option<Vec<Sort>>
}

#[derive(Deserialize, Serialize)]
pub struct Sort {
    property: String,
    direction: String
}


