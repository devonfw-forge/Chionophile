use serde::Serialize;
use suborbital::req;
use suborbital::runnable::*;
use jtq::visitor::logic::api::visitor_eto::VisitorEto;
use suborbital::db;
use suborbital::db::query;

struct GetVisitor{}

impl Runnable for GetVisitor {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        let id = req::url_param("id");
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", id.as_str()));

        match db::select("SelectVisitor", query_args) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &GetVisitor = &GetVisitor{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
