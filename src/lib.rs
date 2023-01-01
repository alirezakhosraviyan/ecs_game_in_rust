extern crate core;

use std::any::Any;

pub mod custom_errors;
mod resource;
mod entity;

use resource::Resource;
use crate::entity::Entity;
use eyre::Result;
use entity::query::Query;

#[derive(Default)]
pub struct World {
    resources: Resource,
    entities: Entity
}

impl World {
    pub fn new () -> Self {
        Self::default()
    }

    /// Adds new resources to the world
    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add_resource(resource);
    }

    /// Returns an immutable representation to the world resources
    pub fn get_resources<T: Any >(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Returns a immutable representation to the world entities
    pub fn get_entities(&self) -> &Entity {
        &self.entities
    }

    /// Returns a mutable reference to the world resources.
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_ref_mut::<T>()
    }

    /// Deletes a resource from the world
    pub fn delete_from_resources<T: Any>(&mut self) -> Result<()> {
        self.resources.delete_resource::<T>()?;
        Ok(())
    }

    /// Register a component to the world
    pub fn register_component<T: Any + 'static>(&mut self) {
        self.entities.register_component::<T>();
    }

    /// Create a new Entity to wold and we can now add new components to that entity
    pub fn create_entity(&mut self) -> &mut Entity {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Result<Query> {
        Ok(Query::new(&self.entities))
    }
}