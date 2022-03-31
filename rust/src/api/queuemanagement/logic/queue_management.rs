use crate::api::queuemanagement::logic::usecase::uc_find_queue::UcFindQueue;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;

pub trait QueueManagement: UcManageQueue + UcFindQueue {}
