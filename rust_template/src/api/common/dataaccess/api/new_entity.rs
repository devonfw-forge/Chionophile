use crate::api::common::logic::api::eto::EntityETO;

/*
    This struct is only needed in those entities where the ID is autogenerated.
    It is intended as a copy of the original Entity without the ID field.
    The struct that implements it will need to derive from Insertable as well.
*/
pub trait NewEntity<ETO>: From<ETO> where ETO: EntityETO {}