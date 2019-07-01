use crate::entity_store::*;
use rand::Rng;

pub struct Game {
    pub entity_store: EntityStore,
    pub rng: rand::rngs::ThreadRng,
}

impl Game {
    fn move_ants(&mut self) {
        let ants = &self.entity_store.ants;
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