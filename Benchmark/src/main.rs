mod models;

use std::sync::Arc;
use goose::prelude::*;
use crate::models::{AccessCodeEto, AccessCodePostData, QueueEto, VisitorEto, VisitorSearchCriteria};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        //.register_taskset(visitor_load_test())
        .register_taskset(logic_load_test())
        .set_default(GooseDefault::Host, "http://localhost:8081/jumpthequeue/services/rest/")?
        .set_default(GooseDefault::RequestLog, "goose-requests.log")?
        .set_default(GooseDefault::ErrorLog, "goose-error.log")?
        .set_default(GooseDefault::Users, 20)?
        .set_default(GooseDefault::HatchRate, "4")?
        .set_default(GooseDefault::RequestBody, true)?
        .set_default(GooseDefault::RunTime, 20)?
        .set_default(GooseDefault::NoResetMetrics, true)?
        .set_default(GooseDefault::StatusCodes, true)?
        .set_default(GooseDefault::ReportFile, "report.html")?
        .set_scheduler(GooseScheduler::Serial)
        .execute()
        .await?
        .print();

    Ok(())
}

/// Tests basic CRUD operations in an entity, in this case Visitor
fn visitor_load_test() -> GooseTaskSet {
    let mut taskset = taskset!("Visitor load tests");
    let visitor_path = "visitormanagement/v1/visitor";

    let request: GooseTaskFunction = Arc::new(move |user| {
        Box::pin( async move {
            let visitor_post = VisitorEto::generate_test_visitor();

            //POST TEST
            let post_response = user.post_json(visitor_path, &visitor_post).await?;
            let visitor_eto = post_response.response?.json::<VisitorEto>().await?;

            let get_by_id_path = format!("{}/{}", visitor_path.clone(), visitor_eto.id.unwrap());

            //GET BY ID TEST
            user.get(&get_by_id_path).await?;

            let search_path = format!("{}/search", visitor_path);
            let search_criteria = VisitorSearchCriteria
            ::generate_test_search_criteria(5, visitor_eto.username, visitor_eto.password);

            //SEARCH TEST
            let _search = user.post_json(&search_path, &search_criteria).await?;

            //DELETE TEST
            user.delete(&get_by_id_path).await?;

            Ok(())
        })
    });
    let task = GooseTask::new(request);
    let new_taskset = taskset.register_task(task);
    taskset = new_taskset;

    taskset
}

/// Tests normal user interaction: Register -> Join Queue -> Leave Queue (-> Delete User)
fn logic_load_test() -> GooseTaskSet {
    let mut taskset = taskset!("Logic load test");
    let accesscode_path = "accesscodemanagement/v1/accesscode/";
    let visitor_path = "visitormanagement/v1/visitor";

    let request: GooseTaskFunction = Arc::new(move |user| {
        Box::pin( async move {

            //Register visitor
            let visitor_post = VisitorEto::generate_test_visitor();
            let post_response = user.post_json(visitor_path, &visitor_post).await?;
            let visitor_eto = post_response.response?.json::<VisitorEto>().await?;

            //Generate accesscode to join the queue
            let accesscode = AccessCodePostData::new(visitor_eto.id.unwrap(), 1);
            let join_queue_response = user.post_json(accesscode_path, &accesscode).await?;
            let accesscode_eto = join_queue_response.response?.json::<AccessCodeEto>().await?;

            //Leave the queue
            let leave_queue_path = format!("{}{}/", accesscode_path.clone(), accesscode_eto.id.unwrap());
            user.delete(&leave_queue_path).await?;

            //Delete user
            let visitor_by_id_path = format!("{}/{}", visitor_path.clone(), visitor_eto.id.unwrap());
            user.delete(&visitor_by_id_path).await?;

            Ok(())
        })
    });
    let task = GooseTask::new(request);
    let new_taskset = taskset.register_task(task);
    taskset = new_taskset;

    taskset
}
