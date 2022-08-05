use crate::{domain::models::new_entity::NewEntity};
use crate::api::model::eto::EntityETO;

pub trait Entity<ID, NE, ETO>: Into<ETO>
    where 
        NE: NewEntity<ETO>, 
        ETO: EntityETO 
{
    fn from_insert(id: ID, new_entity: NE) -> Self;
}
