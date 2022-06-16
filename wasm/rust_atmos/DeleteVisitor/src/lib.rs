use suborbital::runnable::*;
use suborbital::req;
use jtq::common::logic::service::Service;
use jtq::visitor::logic::visitor_service::VisitorService;

struct DeleteVisitor{}

impl Runnable for DeleteVisitor {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");
        let id_num: i64 = id.parse().unwrap_or(-1);
        let delete_result = VisitorService::delete(id_num);
        if let Ok(delete_result) = delete_result {
            Ok(Vec::from(delete_result.to_be_bytes()))
        } else {
            Err(RunErr::new(404, format!("No visitor with id {}", id).as_str()))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &DeleteVisitor = &DeleteVisitor{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
