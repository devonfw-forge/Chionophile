use suborbital::runnable::*;
use suborbital::req;
use jtq::queue::logic::api::queue_eto::QueueEto;
use suborbital::db;
use suborbital::db::query;
use jtq::common::logic::service::Service;
use jtq::queue::logic::queue_service::QueueService;
use anyhow::Result;

struct DeleteQueue{}

impl Runnable for DeleteQueue {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let queue_id: Result<i64> = QueueService::delete(id.parse().unwrap_or(-1));
        return match queue_id {
            Ok(queue_option) =>
                match queue_option {
                    queue => Ok(format!("{}", queue).as_bytes().to_vec()),
                    _ =>  Err(RunErr::new(404, format!("No queue with id {}", id).as_str()))
                }
            Err(e) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}



// initialize the runner, do not edit below //
static RUNNABLE: &DeleteQueue = &DeleteQueue{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
