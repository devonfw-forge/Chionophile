use std::sync::Arc;
use goose::goose::{GooseTask, GooseTaskFunction, GooseTaskSet};
use goose::taskset;
use crate::models::visitor_eto::VisitorEto;
use crate::models::visitor_eto::VisitorPost;
use crate::models::visitor_search_criteria::VisitorSearchCriteria;

/// Tests basic CRUD operations in an entity, in this case Visitor
pub fn taskset() -> GooseTaskSet {
    let mut taskset = taskset!("Visitor load tests");
    let visitor_path = "visitormanagement/v1/visitor/";

    let request: GooseTaskFunction = Arc::new(move |user| {
        Box::pin( async move {
            let visitor_post = VisitorPost::generate_test_visitor();

            //POST TEST
            let post_response = user.post_json(visitor_path, &visitor_post).await?;
            let visitor_eto = post_response.response?.json::<VisitorEto>().await?;

            let get_by_id_path = format!("{}{}/", visitor_path.clone(), visitor_eto.id.unwrap());

            //GET BY ID TEST
            user.get(&get_by_id_path).await?;

            let search_path = format!("{}search/", visitor_path);
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
