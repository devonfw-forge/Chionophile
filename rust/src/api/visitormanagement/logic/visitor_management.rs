use crate::api::visitormanagement::logic::usecase::uc_find_visitor::UcFindVisitor;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;

pub trait VisitorManagement: UcManageVisitor + UcFindVisitor {}
