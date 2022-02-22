use specs::prelude::*;

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl <'a> System<'a> for SysA {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
            println!("pos: {:?}, vel: {:?}", pos, vel);
        }
    }
}

fn main() {
    let mut world = World::new();

    world.register::<Pos>();
    world.register::<Vel>();

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    world.create_entity().with(Pos(2.0)).build();

    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();
    dispatcher.setup(&mut world);
    dispatcher.dispatch(&mut world);
    dispatcher.dispatch(&mut world);
}
