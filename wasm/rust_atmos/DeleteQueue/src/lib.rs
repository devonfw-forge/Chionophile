use suborbital::runnable::*;
use suborbital::req;
use jtq::common::logic::service::Service;
use jtq::queue::logic::queue_service::QueueService;
use anyhow::Result;

struct DeleteQueue{}

impl Runnable for DeleteQueue {
    fn run(&self, _input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let queue_id: Result<i64> = QueueService::delete(id.parse().unwrap_or(-1));
        
        if let Ok(q_id) = queue_id {
            return Ok(q_id.to_string().as_bytes().to_vec())
        }
        else {
            return Err(RunErr::new(404, "No queue"))
        }
    }
}



// initialize the runner, do not edit below //
static RUNNABLE: &DeleteQueue = &DeleteQueue{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
