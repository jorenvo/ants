use crate::entity_store::*;
use rand::prelude::{Rng, SeedableRng};

pub struct Game {
    pub entity_store: EntityStore,
    pub rng: rand::rngs::StdRng,
}

impl Game {
    pub fn init(entity_store: EntityStore) -> Self {
        Self {
            entity_store: entity_store,
            rng: SeedableRng::from_seed([150; 32]),
        }
    }

    fn move_ants(&mut self) {
        let ants = &self.entity_store.ants;
        // make this loop in deterministic order
        for (id, _) in ants {
            if let Some(pos) = self.entity_store.positions.get_mut(id) {
                pos.x += self.rng.gen_range(0, 10);
                pos.y += self.rng.gen_range(0, 10);
            }
        }
    }

    pub fn move_entities(&mut self) {
        self.move_ants();
    }
}