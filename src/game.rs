use crate::components::*;
use crate::entities::*;
use crate::entity_store::*;
use rand::prelude::{SeedableRng, SliceRandom};

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

    fn get_valid_moves(&self, pos: &PositionComponent) -> (Vec<i32>, Vec<i32>) {
        // assume square map
        const MAX: u32 = 11;
        const MOVE_DISTANCE: i32 = 1;
        let mut valid_moves_x: Vec<i32> = vec![];
        let mut valid_moves_y: Vec<i32> = vec![];

        if pos.x.checked_sub(MOVE_DISTANCE as u32).is_some() {
            valid_moves_x.push(-MOVE_DISTANCE);
        }

        if pos.x + (MOVE_DISTANCE as u32) < MAX {
            valid_moves_x.push(MOVE_DISTANCE);
        }

        if pos.y.checked_sub(MOVE_DISTANCE as u32).is_some() {
            valid_moves_y.push(-MOVE_DISTANCE);
        }

        if pos.y + (MOVE_DISTANCE as u32) < MAX {
            valid_moves_y.push(MOVE_DISTANCE);
        }

        (valid_moves_x, valid_moves_y)
    }

    fn move_ants(&mut self) {
        let mut new_positions: Vec<(EntityIndex, PositionComponent)> = vec![];
        for (ant_id, _) in &self.entity_store.ants {
            let mut new_pos = PositionComponent::default();

            if let Some(pos) = self.entity_store.get_position(ant_id) {
                let (valid_moves_x, valid_moves_y) = self.get_valid_moves(pos);
                let x_delta = valid_moves_x.choose(&mut self.rng).unwrap_or(&0);
                let y_delta = valid_moves_y.choose(&mut self.rng).unwrap_or(&0);
                new_pos.x = (pos.x as i32 + x_delta) as u32;
                new_pos.y = (pos.y as i32 + y_delta) as u32;
                new_positions.push((*ant_id, new_pos.clone()));

                let entities_at_new_pos = self.entity_store.get_entities_at(&new_pos);
                if let Some(entities_at_new_pos) = entities_at_new_pos {
                    let pheromones = entities_at_new_pos.iter().filter(|id| {
                        self.entity_store.entity_types.get(id).unwrap() == &EntityType::Pheromone
                    });
                    let sugars = entities_at_new_pos.iter().filter(|id| {
                        self.entity_store.entity_types.get(id).unwrap() == &EntityType::Sugar
                    });

                    for ph in pheromones {
                        println!(
                            "ant {} and pheromone {} at position {:?}",
                            ant_id, ph, new_pos
                        );
                    }

                    for sugar in sugars {
                        println!(
                            "ant {} and sugar {} at position {:?}",
                            ant_id, sugar, new_pos
                        );
                    }
                }
            }
        }

        for (ant_id, pos) in new_positions {
            self.entity_store.update_position(ant_id, &pos);
        }
    }

    pub fn move_entities(&mut self) {
        self.move_ants();
    }
}
