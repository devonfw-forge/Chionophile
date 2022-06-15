use crate::common::logic::entity_eto::EntityETO;

pub trait Entity<ID, ETO>: Into<ETO>
    where
        ETO: EntityETO
{
}
