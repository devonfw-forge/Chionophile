use goose::prelude::*;

/// Task that make a POST request to get the data of 10 visitors
async fn get_10_visitor_data(user: &mut GooseUser) -> GooseTaskResult {
    let json = &serde_json::json!({
        "pageable" : {
            "pageNumber" : 0,
            "pageSize": 10,
            "sort": []
        }
    });
    let _goose = user.post_json("/jumpthequeue/services/rest/visitormanagement/v1/visitor/search", &json).await?;
    //println!("{:?}",_goose.response);
    Ok(())
}

/// Task that make a GET request to obtain the data of a visitor knowing his id
async fn get_visitor_by_id(user: &mut GooseUser) -> GooseTaskResult {
    let _goose = user.get("/jumpthequeue/services/rest/visitormanagement/v1/visitor/1/").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_taskset(taskset!("LoadtestTasks")
            .register_task(task!(get_10_visitor_data))
            .register_task(task!(get_visitor_by_id))
        )
        .set_default(GooseDefault::Host, "http://localhost:8081/")?
        .set_default(GooseDefault::RequestLog, "goose-requests.log")?
        .set_default(GooseDefault::DebugLog, "goose-debug.log")?
        .set_default(GooseDefault::Users, 20)?
        .set_default(GooseDefault::HatchRate, "4")?
        .set_default(GooseDefault::RunTime, 10)?
        .set_default(GooseDefault::NoResetMetrics, true)?
        .set_default(GooseDefault::StatusCodes, true)?
        .execute()
        .await?
        .print();

    Ok(())
}