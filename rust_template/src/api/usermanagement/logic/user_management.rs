use crate::api::usermanagement::logic::usecase::uc_find_user::UcFindUser;
use crate::api::usermanagement::logic::usecase::uc_manage_user::UcManageUser;

pub trait UserManagement: UcManageUser + UcFindUser {}