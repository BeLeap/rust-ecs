use legion::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

fn main() {
    let mut world = World::default();

    let entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));

    let entites: &[Entity] = world.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 2.0, dy: 2.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 4.0, dy: 4.0 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 6.0, dy: 6.0 }),
    ]);

    if let Some(mut entry) = world.entry(entity) {
        println!("{:?} has {:?}", entity, entry.archetype().layout().component_types());

        entry.add_component(12f32);
        assert_eq!(entry.get_component::<f32>().unwrap(), &12f32)
    }

    let mut query = <&Position>::query();

    for position in query.iter(&world) {
        println!("{:?}", position)
    }

    let mut query = <(&Velocity, &mut Position)>::query();

    for (velocity, position) in query.iter_mut(&mut world) {
        position.x += velocity.dx;
        position.y += velocity.dy;
    }

    let mut query = <&Position>::query();

    for position in query.iter(&world) {
        println!("{:?}", position)
    }

}
