use crate::api::common::dataaccess::api::new_entity::NewEntity;
use crate::api::common::logic::api::eto::EntityETO;

pub trait Entity<ID, NE, ETO>: Into<ETO>
    where 
        NE: NewEntity<ETO>, 
        ETO: EntityETO 
{
    fn from_insert(id: ID, new_entity: NE) -> Self;
}
