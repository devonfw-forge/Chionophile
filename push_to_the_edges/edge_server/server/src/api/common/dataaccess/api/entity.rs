use crate::api::common::logic::api::eto::EntityETO;

pub trait Entity<ID, ETO>: Into<ETO>
    where 
        ETO: EntityETO
{
}
