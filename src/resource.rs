use std::any::{Any, TypeId};
use std::collections::HashMap;
use eyre::Result;
use crate::custom_errors::CustomError;


#[derive(Default)]
pub struct Resource {
    /// The data spot in the resource
    data: HashMap<TypeId, Box<dyn Any>>
}

impl Resource {

    /// returns the data spot in the resource
    pub fn get_data(&self) -> &HashMap<TypeId, Box<dyn Any>> {
        &self.data
    }

    /// Adds a value to the resource
    pub fn add_resource(&mut self, data: impl Any) {
        self.data.insert(data.type_id(), Box::new(data));
    }

    /// Returns an immutable representation of the resource
    pub fn get_ref<T: Any>(&self) -> Option::<&T>{
        let type_id = TypeId::of::<T>();
        match self.data.get(&type_id) {
            Some(data) => data.downcast_ref(),
            None => None
        }
    }

    /// Returns a mutable reference to the resource to change the value
    pub fn get_ref_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        match self.data.get_mut(&type_id) {
            Some(res) => res.downcast_mut(),
            _ => None
        }
    }

    /// Deletes the resource data from the hash map
    pub fn delete_resource<T: Any>(&mut self) -> Result<()>{
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id).ok_or(CustomError::CanNotDeleteResource)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct SampleWorld(f64);

    #[test]
    fn test_add_resource() {
        let (resource, type_id) = helper_init_resource();

        let restored_sample_world = resource.get_data().get(&type_id).unwrap().downcast_ref::<SampleWorld>().unwrap();
        assert_eq!(restored_sample_world.0, 500.0);
    }

    #[test]
    fn test_get_ref() {
        let (resource, _) = helper_init_resource();

        match resource.get_ref::<SampleWorld>() {
            Some(ref_res) => assert_eq!(ref_res.0, 500.0),
            _ => panic!("We should have a value for resource data")
        }
    }

    #[test]
    fn test_get_ref_mut() {
        let (mut resource, _) = helper_init_resource();

        match resource.get_ref_mut::<SampleWorld>() {
            Some(data) => data.0 += 1.0,
            None => panic!("We should have a value for resource data")
        }

        // we don't want to test get ref again but we line to get panic if it was none in tests
        let data = resource.get_ref::<SampleWorld>().unwrap();
        assert_eq!(data.0, 501.0);

    }

    #[test]
    fn test_delete_resource() {
        let (mut resource, _) = helper_init_resource();
        resource.delete_resource::<SampleWorld>().unwrap();
        assert!(resource.get_ref::<SampleWorld>().is_none(),  "Resource should be deleted")
    }

    fn helper_init_resource() -> (Resource, TypeId) {
        let mut resource = Resource::default();

        let sample_world = SampleWorld(500.0);
        let sample_world_type_id = sample_world.type_id();
        resource.add_resource(sample_world);

        (resource, sample_world_type_id)
    }

}