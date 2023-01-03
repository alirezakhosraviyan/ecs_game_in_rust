use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use eyre::Result;
use crate::custom_errors::CustomError;
use crate::entity::Entity;

#[derive(Debug)]
pub struct Query<'a> {
    map: u32,
    entities: &'a Entity,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entity) -> Self {
        Self { entities, map: 0, type_ids: vec![] }
    }
    pub fn with_component<T: Any + 'static>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        let bitmask = self.entities.get_bitmask(&type_id).ok_or(CustomError::UnregisteredComponentAdded)?;
        self.map |= bitmask;
        self.type_ids.push(type_id);
        Ok(self)
    }

    pub fn run(&self) -> Result<Vec<Vec<Rc<RefCell<dyn Any>>>>> {

        let indices = self.entities.bitmap.iter().enumerate().filter_map(|(index, value)| match value & self.map {
            value if value == self.map  => Some(index),
            _ => None
        }).collect::<Vec<usize>>();
        let mut result  = vec![];

        for type_id in self.type_ids.iter() {
            let entity_component = self.entities.get_component(type_id);
            let mut component_to_keep = vec![];

            for index in &indices {
                component_to_keep.push(entity_component[*index].as_ref().unwrap().clone())
            }
            result.push(component_to_keep);
        }
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_component() -> Result<()> {
        let mut entities = Entity::default();
        entities.register_component::<u32>();
        entities.register_component::<f64>();

        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f64>()?;

        assert_eq!(query.map, 3);
        assert_eq!(query.type_ids[0], TypeId::of::<u32>());
        assert_eq!(query.type_ids[1], TypeId::of::<f64>());
        Ok(())
    }

    #[test]
    fn test_run_query() -> Result<()> {
        let mut entities = Entity::default();
        entities.register_component::<u32>();
        entities.register_component::<f64>();
        entities.create_entity().with_component(10_u32)?.with_component(20.0_f64)?;
        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(50.0_f64)?;
        entities.create_entity().with_component(15_u32)?.with_component(25.0_f64)?;

        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f64>()?;

        let result = query.run().unwrap();
        let u32s = &result[0];
        let f64s = &result[1];

        assert_eq!(u32s.len(), f64s.len());
        assert_eq!(u32s.len(), 2);

        let first_u32 = u32s[0].borrow();
        let val_first_u32 = first_u32.downcast_ref::<u32>().unwrap();

        assert_eq!(*val_first_u32, 10_u32);

        let first_f64 = f64s[1].borrow();
        let val_first_f64 = first_f64.downcast_ref::<f64>().unwrap();

        assert_eq!(*val_first_f64, 25.0_f64);


        Ok(())
    }
}