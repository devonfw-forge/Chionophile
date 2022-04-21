use std::sync::Arc;
use goose::goose::{GooseTask, GooseTaskFunction, GooseTaskSet};
use goose::taskset;
use crate::models::accesscode_eto::AccessCodeEto;
use crate::models::accesscode_post_data::AccessCodePostData;
use crate::models::visitor_eto::VisitorEto;
use crate::models::visitor_eto::VisitorPost;

/// Tests normal user interaction: Register -> Join Queue -> Leave Queue (-> Delete User)
pub fn taskset() -> GooseTaskSet {
    let mut taskset = taskset!("Logic load test");
    let accesscode_path = "accesscodemanagement/v1/accesscode/";
    let visitor_path = "visitormanagement/v1/visitor/";

    let request: GooseTaskFunction = Arc::new(move |user| {
        Box::pin( async move {

            //Register visitor
            let visitor_post = VisitorPost::generate_test_post_visitor();
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
            let visitor_by_id_path = format!("{}{}/", visitor_path.clone(), visitor_eto.id.unwrap());
            user.delete(&visitor_by_id_path).await?;

            Ok(())
        })
    });
    let task = GooseTask::new(request);
    let new_taskset = taskset.register_task(task);
    taskset = new_taskset;

    taskset
}
