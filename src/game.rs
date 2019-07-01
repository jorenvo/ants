#![deny(clippy::pedantic)]
use crate::components::*;
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
        for (ant_id, _) in ants {
            let mut updated_position = false;
            let mut new_pos = PositionComponent::default();
            if let Some(pos) = self.entity_store.positions.get(ant_id) {
                updated_position = true;
                new_pos.x = pos.x + self.rng.gen_range(0, 10);
                new_pos.y = pos.y + self.rng.gen_range(0, 10);

                if let Some(ph_id) = self.entity_store.get_pheromone_at(&new_pos) {
                    println!(
                        "ant {} and pheromone {} at position {:?}",
                        ant_id, ph_id, new_pos
                    );
                }
            }

            if updated_position {
                self.entity_store.positions.insert(*ant_id, new_pos);
            }
        }
    }

    pub fn move_entities(&mut self) {
        self.move_ants();
    }
}