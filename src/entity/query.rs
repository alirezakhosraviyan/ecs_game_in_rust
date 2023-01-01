use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use eyre::Result;
use crate::custom_errors::CustomError;
use crate::entity::Entity;

#[derive(Debug)]
pub struct Query <'a> {
    map: u32,
    entities: &'a Entity,
    type_ids: Vec<TypeId>
}

impl <'a> Query <'a> {

    pub fn new(entities: &'a Entity) -> Self {
        Self {entities, map: 0, type_ids: vec![]}
    }
    pub fn with_component<T: Any + 'static>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        let bitmask = self.entities.get_bitmask(&type_id).ok_or(CustomError::UnregisteredComponentAdded)?;
        self.map |= bitmask;
        self.type_ids.push(type_id);
        Ok(self)
    }

    pub fn run(&self) -> Result<Vec<Vec<Rc<RefCell<dyn Any>>>>> {
        Ok(vec![])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent(u8);

    fn test_with_component() -> Result<()> {
        let mut entities = Entity::default();
        entities.register_component::<u32>();
        entities.register_component::<f64>();

        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f64>();

        assert_eq!(query.map, 3);
        assert_eq!(query.type_ids[0], TypeId::of::<u32>());
        assert_eq!(query.type_ids[1], TypeId::of::<f64>());
        Ok(())
    }
}