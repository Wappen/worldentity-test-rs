use crate::entity::Entity;
use crate::world::World;
use std::thread::sleep;
use std::time::Duration;

mod entity;
mod world;

fn main() {
    let mut world = World::new();
    world.spawn(Entity::new("Herr Knipper"));
    world.spawn(Entity::new("Herr Korstian"));
    world.spawn(Entity::new("Herr Willms"));
    world.spawn(Entity::new("Frau Rohlfs"));
    world.spawn(Entity::new("Frau Barth"));

    loop {
        world.do_tick();
        sleep(Duration::from_secs(1));
    }
}
