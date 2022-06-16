use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::visitor::logic::api::visitor_eto::VisitorEto;
use suborbital::db;
use suborbital::db::query;
use jtq::common::logic::service::Service;
use jtq::visitor::logic::visitor_service::VisitorService;
use anyhow::Result;

struct GetVisitor{}

impl Runnable for GetVisitor {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");

        let visitor_eto: Result<Option<Vec<u8>>> = VisitorService::get_by_id(id.parse().unwrap_or(-1));
        return match visitor_eto {
            Ok(visitor_option) =>
                match visitor_option {
                    Some(visitor) => Ok(visitor),
                    _ =>  Err(RunErr::new(404, format!("No visitor with id {}", id).as_str()))
                }
            Err(e) => Err(RunErr::new(500, "Internal Server Error"))
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &GetVisitor = &GetVisitor{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
