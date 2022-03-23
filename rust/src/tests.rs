use super::*;
use actix_web::{test, web, App};
use actix_web::test::TestRequest;
use actix_web::web::Buf;
use prost::bytes;
use crate::accesscode_services::generate_ticket_code;
use crate::models::{AccessCodeEto, Pageable, VisitorEto, VisitorSearchCriteria};


#[actix_rt::test]
async fn test_access_code() {
    let manager = ConnectionManager::<SqliteConnection>::new("C:\\Users\\airherna\\Projects\\jtqt-rust\\jtqt.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let cors = Cors::permissive();
    let app = App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(cors)
        .wrap(middleware::Logger::default())
        .service(web::scope("jumpthequeue/services/rest/visitormanagement/v1").configure(visitor_services::visitor_scope))
        .service(web::scope("jumpthequeue/services/rest/queuemanagement/v1").configure(queue_services::queue_scope))
        .service(web::scope("jumpthequeue/services/rest/accesscodemanagement/v1").configure(accesscode_services::access_code_scope));

    let mut test_service = test::init_service(app).await;

    let filters = models::AccessCodeSearchCriteria {
        ticketNumber: None,
        creationTime: None,
        startTime: None,
        endTime: None,
        visitorId: None,
        queueId: None,
        pageable: Pageable {
            pageSize: 1,
            pageNumber: 0,
            sort: None,
        },
    };

    let req = test::TestRequest::post()
        .uri("/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/cto/search")
        .set_json(&filters)
        .to_request();

    println!("{:?}", req);

    let resp = test::call_service(&mut test_service, req).await;

    println!("{} {}", resp.status().is_success(), resp.status());
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn register_test() {
    use super::*;
    use actix_web::{body::Body, test, web, App};
    use serde_json::json;
    use bytes::Bytes;

    let manager = ConnectionManager::<SqliteConnection>::new("C:\\Users\\airherna\\Projects\\jtqt-rust\\jtqt.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let cors = Cors::permissive();
    let app = App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(cors)
        .wrap(middleware::Logger::default())
        .service(web::scope("jumpthequeue/services/rest/visitormanagement/v1").configure(visitor_services::visitor_scope))
        .service(web::scope("jumpthequeue/services/rest/queuemanagement/v1").configure(queue_services::queue_scope))
        .service(web::scope("jumpthequeue/services/rest/accesscodemanagement/v1").configure(accesscode_services::access_code_scope));

    let mut test_service = test::init_service(app).await;

    let visitor_search = VisitorSearchCriteria {
        pageable: Pageable {
            pageSize: 1,
            pageNumber: 0,
            sort: None,
        },
        username: Some("testUser991".to_string()),
        password: Some("password".to_string()),
    };
    let new_visitor = VisitorEto {
        id: None,
        username: "testUser991".to_string(),
        name: "test".to_string(),
        phoneNumber: "test".to_string(),
        password: Some("password".to_string()),
        acceptedCommercial: Some(true),
        acceptedTerms: Some(true),
        userType: Some(false),
    };

    let search_request = TestRequest::post().uri(
        "/jumpthequeue/services/rest/visitormanagement/v1/visitor/search"
    ).set_json(&visitor_search).to_request();

    let resp = test::call_service(&mut test_service, search_request).await;
    assert!(resp.status().is_success());
    let body_bytes = test::read_body(resp).await;
    //No user
    assert_eq!(b"{\"content\":[],\"pageable\":{\"pageSize\":1,\"pageNumber\":0,\"sort\":null},\"totalElements\":0}", body_bytes.bytes());

    let register_request = TestRequest::post().uri(
        "/jumpthequeue/services/rest/visitormanagement/v1/visitor"
    ).set_json(&new_visitor).to_request();

    let resp = test::call_service(&mut test_service, register_request).await;
    assert!(resp.status().is_success());
    let body_bytes = test::read_body(resp).await;
    //Now this should not be the response and there should be a user
    assert_ne!(b"{\"content\":[],\"pageable\":{\"pageSize\":1,\"pageNumber\":0,\"sort\":null},\"totalElements\":0}", body_bytes.bytes());
}

#[actix_rt::test]
async fn correct_number_assingment() {
    let mut access_code_eto = AccessCodeEto {
        id: None,
        ticketNumber: Some("Q000".to_string()),
        creationTime: None,
        startTime: None,
        endTime: None,
        queueId: None,
        visitorId: None,
    };
    assert_eq!(generate_ticket_code(&access_code_eto), "Q001");

    access_code_eto.ticketNumber = Some("Q999".to_string());
    assert_eq!(generate_ticket_code(&access_code_eto), "Q000");

    access_code_eto.ticketNumber = Some("Q009".to_string());
    assert_eq!(generate_ticket_code(&access_code_eto), "Q010");
}

