use std::sync::Arc;
use goose::goose::{GooseTask, GooseTaskFunction, GooseTaskSet};
use goose::taskset;
use crate::models::accesscode_eto::AccessCodeEto;
use crate::models::accesscode_post_data::AccessCodePostData;
use crate::models::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::models::queue_search_criteria::QueueSearchCriteria;
use crate::models::visitor_eto::VisitorEto;
use crate::models::visitor_eto::VisitorPost;
use crate::models::visitor_search_criteria::VisitorSearchCriteria;

/// Tests normal user interaction: Register -> Join Queue -> Leave Queue (-> Delete User)
pub fn taskset() -> GooseTaskSet {
    let mut taskset = taskset!("Logic load test");
    let accesscode_path = "accesscodemanagement/v1/accesscode/";
    let visitor_path = "visitormanagement/v1/visitor/";
    let queue_path = "queuemanagement/v1/queue/search/";

    let request: GooseTaskFunction = Arc::new(move |user| {
        Box::pin( async move {

            //Register visitor
            let visitor_post = VisitorPost::generate_test_post_visitor();
            let post_response = user.post_json(visitor_path, &visitor_post).await.expect("json parse not worky");
            println!("{:#?}", post_response.response?.text().await);
            // let visitor_eto = post_response.response.expect("visitor_eto_response").json::<VisitorEto>().await.expect("VisitorEto");


            //Generate accesscode to join the queue
            // let accesscode = AccessCodePostData::new(visitor_eto.id.unwrap(), 1);
            // let join_queue_response = user.post_json(accesscode_path, &accesscode).await.expect("join_queue_response");
            // println!("{:#.expect("json parse not worky")}", join_queue_response.response.expect("json parse not worky").text().await);
            // let accesscode_eto = join_queue_response.response.expect("join_queue_response").json::<AccessCodeEto>().await.expect("AccessCodeEto");

            //Search accesscodess
            let accesscode_search_path = format!("{}search/", accesscode_path);
            let _accesscode_search = user.post_json(&accesscode_search_path, &AccessCodeSearchCriteria::generate_test_struct(40)).await.expect("_accesscode_search");

            //Search visitors
            let search_path = format!("{}search/", visitor_path);
            let search_criteria = VisitorSearchCriteria::generate_test_search_criteria(40, None, None);
            let _search = user.post_json(&search_path, &search_criteria).await.expect("_search");

            //Search queues
            let queue_criteria = QueueSearchCriteria::generate_test_struct(40);
            let _search_queue = user.post_json(&queue_path, &queue_criteria).await.expect("_search_queue");

            //Leave the queue
            // let leave_queue_path = format!("{}{}/", accesscode_path.clone(), accesscode_eto.id.unwrap());
            // user.delete(&leave_queue_path).await.expect("user.delete");

            //Delete user
            // let visitor_by_id_path = format!("{}{}/", visitor_path.clone(), visitor_eto.id.unwrap());
            // user.delete(&visitor_by_id_path).await.expect("visitor_by_id_path");

            Ok(())
        })
    });
    let task = GooseTask::new(request);
    let new_taskset = taskset.register_task(task);
    taskset = new_taskset;

    taskset
}
