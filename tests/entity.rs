use std::any::{TypeId};
use ecs_game_in_rust::World;

struct TestComponent(u8);

#[test]
fn test_create_entity() {
    let mut world = World::new();
    let test_component = TestComponent(8);
    world.register_component::<TestComponent>();
    world.create_entity().with_component(test_component).unwrap();

    let entity = world.get_entities().components.get(&TypeId::of::<TestComponent>());
    assert!(entity.is_some(), "entity is not added to the world");

    let added_component_holder = entity.unwrap()[0].as_ref().unwrap().borrow();
    let added_component =  added_component_holder.downcast_ref::<TestComponent>().unwrap();

    assert_eq!(added_component.0, 8_u8, "Wrong component add to the world");
}