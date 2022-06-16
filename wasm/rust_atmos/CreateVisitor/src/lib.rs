use suborbital::runnable::*;
use jtq::common::logic::service::Service;
use jtq::visitor::logic::visitor_service::VisitorService;

struct CreateVisitor{}

impl Runnable for CreateVisitor {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let created_visitor = VisitorService::create(input);
        if let Ok(created_visitor) = created_visitor {
            Ok(created_visitor)
        } else {
            Err(RunErr::new(500, "Error creating Visitor"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &CreateVisitor = &CreateVisitor{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
