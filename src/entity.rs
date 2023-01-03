pub mod query;

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::custom_errors::CustomError;
use eyre::Result;

#[derive(Default, Debug)]
pub struct Entity {
    pub components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
    pub bitmask: HashMap<TypeId, u32>,
    pub bitmap: Vec<u32>,
}

impl Entity {
    pub fn register_component<T: Any + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, vec![]);
        self.bitmask.insert(type_id, 2u32.pow(self.bitmask.keys().len() as u32));
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.components.iter_mut().for_each(|(_, component)| component.push(None));
        self.bitmap.push(0);
        self
    }

    pub fn with_component(&mut self, component: impl Any) -> Result<&mut Self> {
        let data = self.components.get_mut(&component.type_id()).ok_or(CustomError::UnregisteredComponentAdded)?;
        let bitmask_index = self.bitmask.get(&component.type_id()).unwrap();

        let val = data.last_mut().ok_or(CustomError::CreateComponentNeverCalled)?;
        *val = Some(Rc::new(RefCell::new(component)));

        let last_bitmap = self.bitmap.last_mut().unwrap();
        *last_bitmap |= *bitmask_index;

        Ok(self)
    }

    pub fn get_bitmask(&self, type_id: &TypeId) -> Option<u32> {
        self.bitmask.get(type_id).copied()
    }

    pub fn get_component(&self, type_id: &TypeId) -> &Vec<Option<Rc<RefCell<dyn Any>>>> {
        self.components.get(type_id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent(u8);

    struct Test2Component(u8);

    #[test]
    fn test_register_component() {
        let mut entity = Entity::default();
        entity.register_component::<TestComponent>();
        let restored_component = entity.components.get(&TypeId::of::<TestComponent>()).unwrap();
        assert!(restored_component.is_empty(), "Entity is not initialized correctly");
    }

    #[test]
    fn test_component_to_be_masked() {
        let mut entity = Entity::default();
        entity.register_component::<TestComponent>();
        let mask = entity.bitmask.get(&TypeId::of::<TestComponent>()).unwrap();
        assert_eq!(*mask, 1);

        entity.register_component::<Test2Component>();
        let mask = entity.bitmask.get(&TypeId::of::<Test2Component>()).unwrap();
        assert_eq!(*mask, 2);
    }


    #[test]
    fn create_entity() {
        let mut entity = Entity::default();
        entity.register_component::<TestComponent>();
        entity.create_entity();
        let restored_component = entity.components.get(&TypeId::of::<TestComponent>()).unwrap();
        assert!(restored_component.len() == 1 && restored_component[0].is_none(), "Entity is not initialized correctly");
    }

    #[test]
    fn test_with_component() {
        let mut entity = Entity::default();
        let test_component = TestComponent(12);
        let test2_component = Test2Component(13);
        entity.register_component::<TestComponent>();
        entity.register_component::<Test2Component>();
        entity.create_entity().with_component(test_component).unwrap().with_component(test2_component).unwrap();

        // because of "temporary value rule" we should split the line into two variables if we want to have the result of borrow function to be alive more.
        let reference_to_component = entity.components.get(&TypeId::of::<TestComponent>()).unwrap()[0].as_ref().unwrap().borrow();
        let restored_component = reference_to_component.downcast_ref::<TestComponent>().unwrap();

        assert_eq!(restored_component.0, 12, "Component is not stored correctly");
    }

    #[test]
    fn test_with_unregistered_component() {
        let mut entity = Entity::default();
        let test_component = TestComponent(12);
        let is_error = entity.create_entity().with_component(test_component).is_err();
        assert!(is_error);
    }

    #[test]
    fn test_with_component_to_update_bitmap() {
        let mut entity = Entity::default();
        let test_component = TestComponent(12);
        let test2_component = Test2Component(13);
        entity.register_component::<TestComponent>();
        entity.register_component::<Test2Component>();
        entity.create_entity().with_component(test_component).unwrap().with_component(test2_component).unwrap();

        let bitmap_entity = &entity.bitmap[0];

        // because it has two component then we would have 0b00000001 + 0b0000010 => 0b00000011 = 3
        assert_eq!(*bitmap_entity, 3);
        let test3_component = TestComponent(88);
        entity.create_entity().with_component(test3_component).unwrap();

        let bitmap_entity = &entity.bitmap[1];

        // because it has two component then we would have 0b00000001 + 0b0000010 => 0b00000011 = 3
        assert_eq!(*bitmap_entity, 1);
    }
}