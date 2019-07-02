use crate::components::*;
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
        const MAX: u32 = 10;
        const MOVE_DISTANCE: i32 = 2;
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
        let ants = &self.entity_store.ants;
        for (ant_id, _) in ants {
            let mut updated_position = false;
            let mut new_pos = PositionComponent::default();

            if let Some(pos) = self.entity_store.positions.get(ant_id) {
                let (valid_moves_x, valid_moves_y) = self.get_valid_moves(pos);
                println!(
                    "current y: {:?}, valid moves: {:?} {:?}",
                    pos, valid_moves_x, valid_moves_y
                );

                let x_move = valid_moves_x.choose(&mut self.rng).unwrap_or(&0);
                let y_move = valid_moves_y.choose(&mut self.rng).unwrap_or(&0);
                new_pos.x = (pos.x as i32 + x_move) as u32;
                new_pos.y = (pos.y as i32 + y_move) as u32;
                updated_position = true;

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