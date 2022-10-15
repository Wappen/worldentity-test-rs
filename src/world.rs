use crate::entity::Entity;
use rand::seq::IteratorRandom;
use std::cell::RefCell;

pub struct World {
    entities: Vec<RefCell<Entity>>,
}

impl World {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn do_tick(&self) {
        let samples = &self
            .entities
            .iter()
            .choose_multiple(&mut rand::thread_rng(), 2);

        let mut entity1 = samples[0].borrow_mut();
        let mut entity2 = samples[1].borrow_mut();

        Entity::fight(&mut entity1, &mut entity2);
    }

    pub fn spawn(&mut self, entity: Entity) {
        self.entities.push(RefCell::from(entity));
    }
}
