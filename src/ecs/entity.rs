type EntityID = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity {
    pub id: EntityID
}