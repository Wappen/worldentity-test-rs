use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Entity {
    id: u64,
    hp: i32,
    name: String,
}

static BASE_ID: AtomicU64 = AtomicU64::new(0);

impl PartialEq<Self> for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl Entity {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: Self::new_id(),
            hp: 100,
            name: name.into(),
        }
    }

    pub fn fight(e1: &mut Entity, e2: &mut Entity) {
        let (attacker, victim) = {
            if thread_rng().gen_bool(0.5) {
                (e1, e2)
            } else {
                (e2, e1)
            }
        };

        let damage_dealt = attacker.attack(victim);
        println!(
            "{} attacked {} and dealt {} damage.",
            &attacker.name, &victim.name, damage_dealt
        );
    }

    pub fn attack(&self, victim: &mut Entity) -> u64 {
        victim.hp -= 1;
        1
    }

    fn new_id() -> u64 {
        let id = BASE_ID.load(Ordering::Relaxed);
        BASE_ID.store(id + 1, Ordering::Relaxed);
        id
    }
}
