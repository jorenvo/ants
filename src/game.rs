use crate::components::*;
use crate::entities::*;
use crate::entity_store::*;
use rand::prelude::{SeedableRng, SliceRandom};

// TODO is this a System?
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

    fn handle_new_ant_pos(&mut self, ant_id: &EntityIndex, new_pos: &PositionComponent) {
        let entities_at_new_pos = self.entity_store.get_entities_at(&new_pos);
        let mut new_releasing_ph_components: Vec<(EntityIndex, ReleasingPheromoneComponent)> =
            vec![];

        if let Some(entities_at_new_pos) = entities_at_new_pos {
            for id in entities_at_new_pos {
                match self.entity_store.entity_types.get(id) {
                    Some(EntityType::Pheromone) => {
                        println!(
                            "ant {} and pheromone {} at position {:?}",
                            ant_id, id, new_pos
                        );
                    }
                    Some(EntityType::Sugar) => {
                        println!("ant {} and sugar {} at position {:?}", ant_id, id, new_pos);
                        new_releasing_ph_components.push((
                            *ant_id,
                            ReleasingPheromoneComponent {
                                ticks_left: 4,
                                ph_type: PheromoneType::Food,
                            },
                        ));
                    }
                    _ => {}
                }
            }
        }

        for (id, comp) in new_releasing_ph_components {
            self.entity_store.releasing_pheromones.insert(id, comp);
        }
    }

    fn release_pheromones(&mut self, ant_id: &EntityIndex) {
        if let Some(releasing_pheromone_comp) =
            self.entity_store.releasing_pheromones.get_mut(ant_id)
        {
            releasing_pheromone_comp.ticks_left -= 1;
            if releasing_pheromone_comp.ticks_left == 0 {
                self.entity_store.releasing_pheromones.remove(ant_id);
            } else {
                match releasing_pheromone_comp.ph_type {
                    PheromoneType::Food => {
                        let ph_id = self.entity_store.create_entity(&EntityType::Pheromone);
                        self.entity_store
                            .intensities
                            .insert(ph_id, IntensityComponent { strength: 8 });

                        let ant_pos = self.entity_store.get_position(ant_id).unwrap().clone();
                        self.entity_store.update_position(ph_id, &ant_pos);
                    }
                }
            }
        }
    }

    fn ants(&mut self) {
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
            }
        }

        for (ant_id, pos) in new_positions {
            self.entity_store.update_position(ant_id, &pos);
            self.handle_new_ant_pos(&ant_id, &pos);
            self.release_pheromones(&ant_id);
        }
    }

    fn pheromones(&mut self) {
        dbg!(&self.entity_store.pheromones);
        dbg!(&self.entity_store.intensities);
        for (ph_id, _) in &self.entity_store.pheromones {
            let intensity = self.entity_store.intensities.get_mut(&ph_id).unwrap();
            intensity.strength -= 1;

            if intensity.strength == 0 {
                self.entity_store.intensities.remove(&ph_id);
            }
        }
    }

    pub fn tick(&mut self) {
        self.ants();
        self.pheromones();
    }
}
