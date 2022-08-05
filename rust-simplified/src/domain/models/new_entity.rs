use crate::api::model::eto::EntityETO;

pub trait NewEntity<ETO>: From<ETO> where ETO: EntityETO {}