use jtq::{queue::logic::queue_service::QueueService, common::logic::service::Service};
use suborbital::runnable::*;

struct CreateQueue{}

impl Runnable for CreateQueue {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let created_queue = QueueService::create(input);
        if let Ok(created_queue) = created_queue {
            Ok(created_queue)
        } else {
            Err(RunErr::new(500, "Error creating Queue"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &CreateQueue = &CreateQueue{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
