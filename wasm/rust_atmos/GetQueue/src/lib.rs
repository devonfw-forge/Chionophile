use suborbital::runnable::*;
use suborbital::req;
use jtq::common::logic::service::Service;
use jtq::queue::logic::queue_service::QueueService;
use anyhow::Result;

struct GetQueue{}

impl Runnable for GetQueue {
    fn run(&self, _input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let queue_eto: Result<Option<Vec<u8>>> = QueueService::get_by_id(id.parse().unwrap_or(-1));
        return match queue_eto {
            Ok(queue_option) =>
                match queue_option {
                    Some(queue) => Ok(queue),
                    _ =>  Err(RunErr::new(404, format!("No queue with id {}", id).as_str()))
                }
            Err(_) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}



// initialize the runner, do not edit below //
static RUNNABLE: &GetQueue = &GetQueue{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
