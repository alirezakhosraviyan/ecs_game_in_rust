use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use ecs_game_in_rust::World;
use eyre::Result;

#[derive(Default)]
struct SampleResource(u8);

#[test]
fn test_resources_immutably() {
    let world = init_world();

    match world.get_resources::<SampleResource>() {
        Some(restored_world_resources) => assert_eq!(restored_world_resources.0, 12),
        _ => panic!("We should have a value for the resource")
    }
}

#[test]
fn test_resources_mutably() {
    let mut world = init_world();

    match world.get_resource_mut::<SampleResource>() {
        Some(data) => data.0 = 99,
        _ => panic!("We should have a value for the resource")
    }

    let restored_world_resources = world.get_resources::<SampleResource>().unwrap();
    assert_eq!(restored_world_resources.0, 99)
}

#[test]
fn test_delete_ok() {
    let mut world = init_world();

    let delete_result = world.delete_from_resources::<SampleResource>();
    assert!(delete_result.is_ok(), "Delete failed for World");

    let restored_resource = world.get_resources::<SampleResource>();
    assert!(restored_resource.is_none(), "Resource has not been deleted");
}

fn init_world() -> World {
    let mut world = World::new();
    let sample_resource = SampleResource(12);
    world.add_resource(sample_resource);
    world
}

struct Location(f32, f32);

struct Size(f32);

//
// #[test]
// fn test_query_ok() -> Result<()> {
//     let mut world = init_world();
//     world.register_component::<Location>();
//     world.register_component::<Size>();
//
//     world.create_entity()
//         .with_component(Location(45.22, 25.77))?
//         .with_component(Size(22.0))?;
//
//     world.create_entity()
//         .with_component(Location(46.22, 26.77))?;
//
//     world.create_entity()
//         .with_component(Size(27.0))?;
//
//     world.create_entity()
//         .with_component(Location(48.22, 28.77))?
//         .with_component(Size(28.0))?;
//
//     let query: Vec<Vec<Rc<RefCell<dyn Any>>>> = world
//         .query()?
//         .with_component::<Location>()?
//         .with_component::<Size>()?
//         .run()?;
//
//     let locations: &Vec<Rc<RefCell<dyn Any>>> = &query[0];
//     let sizes: &Vec<Rc<RefCell<dyn Any>>> = &query[1];
//
//     assert_eq!(locations.len(), sizes.len());
//     assert_eq!(locations.len(), 2);
//
//     let first_location_wrapper  = locations[0].borrow();
//     let first_location = first_location_wrapper.downcast_ref::<Location>().unwrap();
//     assert_eq!((first_location.0, first_location.1), (45.22, 25.77) );
//
//     let first_size_wrapper = sizes[0].borrow();
//     let first_size = first_size_wrapper.downcast_ref::<Size>().unwrap();
//
//     assert_eq!(first_size.0, 22.0);
//
//     let second_location_wrapper = locations[1].borrow();
//     let second_location = second_location_wrapper.downcast_ref::<Location>().unwrap();
//     assert_eq!(second_location.0, 46.22);
//
//     let second_size_wrapper = sizes[1].borrow();
//     let second_size = second_size_wrapper.downcast_ref::<Size>().unwrap();
//     assert_eq!(second_size.0, 27.0);
//
//     Ok(())
// }