use crate::api::accesscodemanagement::logic::usecase::uc_find_accesscode::UcFindAccessCode;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;

pub trait AccessCodeManagement: UcManageAccessCode + UcFindAccessCode {}
