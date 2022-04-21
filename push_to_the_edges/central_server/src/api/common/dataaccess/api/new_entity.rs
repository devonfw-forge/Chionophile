use crate::api::common::logic::api::eto::EntityETO;

pub trait NewEntity<ETO>: From<ETO> where ETO: EntityETO {}