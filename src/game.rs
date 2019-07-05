use crate::components::*;
use crate::entities::*;
use crate::entity_store::*;
use colored::*;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::prelude::SeedableRng;
use std::fmt;

// TODO is this a System?
pub struct Game {
    width: f64,
    height: f64,
    pub entity_store: EntityStore,
    pub rng: rand::rngs::StdRng,
}

impl Game {
    pub fn init(entity_store: EntityStore, width: f64, height: f64) -> Self {
        Self {
            width: width,
            height: height,
            entity_store: entity_store,
            rng: SeedableRng::from_seed([150; 32]),
        }
    }

    fn get_weighted_valid_moves(
        &self,
        pos: &PositionComponent,
        direction: &DirectionComponent,
    ) -> Vec<(u32, PositionComponent)> {
        // assume square map
        const MOVE_DISTANCE: f64 = 1.0;
        let mut valid_moves = vec![];

        // Up
        if pos.y - MOVE_DISTANCE >= 0.0 {
            valid_moves.push((
                0,
                PositionComponent {
                    x: 0.0,
                    y: -MOVE_DISTANCE,
                },
            ));
        }

        // Down
        if pos.y + MOVE_DISTANCE < self.height {
            valid_moves.push((
                0,
                PositionComponent {
                    x: 0.0,
                    y: MOVE_DISTANCE,
                },
            ));
        }

        // Left
        if pos.x - MOVE_DISTANCE >= 0.0 {
            valid_moves.push((
                0,
                PositionComponent {
                    x: -MOVE_DISTANCE,
                    y: 0.0,
                },
            ));
        }

        // Right
        if pos.x + MOVE_DISTANCE < self.width {
            valid_moves.push((
                0,
                PositionComponent {
                    x: MOVE_DISTANCE,
                    y: 0.0,
                },
            ));
        }

        // 80%: same direction
        //  9%: go right
        //  9%: go left
        //  2%: go back
        for m in valid_moves.iter_mut() {
            if m.1.x == direction.x && m.1.y == direction.y {
                m.0 = 60;
            } else if m.1.x == direction.x || m.1.y == direction.y {
                m.0 = 18;
            } else {
                m.0 = 4;
            }
        }

        valid_moves
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
                                ticks_left: 8,
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

    fn merge_and_clear_pheromones(
        &mut self,
        pos: &PositionComponent,
        extra_strength: u32,
    ) -> IntensityComponent {
        let mut intensity = IntensityComponent {
            strength: extra_strength,
        };
        let mut pheromones_to_delete = vec![];

        if let Some(entities) = self.entity_store.get_entities_at(pos) {
            let pheromones: Vec<&EntityIndex> = entities
                .iter()
                .filter(|id| {
                    self.entity_store.entity_types.get(id).unwrap() == &EntityType::Pheromone
                })
                .collect();

            if !pheromones.is_empty() {
                let merged_strength: u32 = pheromones
                    .iter()
                    .map(|p| self.entity_store.intensities.get(p).unwrap().strength)
                    .sum();
                intensity.strength += merged_strength;

                pheromones_to_delete = pheromones.iter().map(|p| **p).collect();
            }
        }

        for ph in pheromones_to_delete {
            self.entity_store.remove_position(&ph);
            self.entity_store.intensities.remove(&ph);
            self.entity_store.pheromones.remove(&ph);
        }

        intensity
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
                        let ant_pos = self.entity_store.get_position(ant_id).unwrap().clone();
                        let intensity = self.merge_and_clear_pheromones(&ant_pos, 8);

                        // TODO create pheromone_types component BTreeMap in entity store
                        let ph_id = self.entity_store.create_entity(&EntityType::Pheromone);
                        self.entity_store.update_position(&ph_id, &ant_pos);
                        self.entity_store.intensities.insert(ph_id, intensity);
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
                let direction = self
                    .entity_store
                    .get_direction(ant_id)
                    .unwrap_or(&DirectionComponent { x: 1.0, y: 0.0 });
                let valid_moves = self.get_weighted_valid_moves(pos, direction);
                let weighted_index =
                    WeightedIndex::new(valid_moves.iter().map(|item| item.0)).unwrap();
                let random_move = &valid_moves[weighted_index.sample(&mut self.rng)].1;
                new_pos.x = pos.x + random_move.x;
                new_pos.y = pos.y + random_move.y;
                new_positions.push((*ant_id, new_pos.clone()));
            }
        }

        for (ant_id, pos) in new_positions {
            self.entity_store.update_position(&ant_id, &pos);
            self.handle_new_ant_pos(&ant_id, &pos);
            self.release_pheromones(&ant_id);
        }
    }

    fn pheromones(&mut self) {
        let mut evaporated_pheromones = vec![];
        for (ph_id, _) in &self.entity_store.pheromones {
            let intensity = self.entity_store.intensities.get_mut(&ph_id).unwrap();
            intensity.strength -= 1;

            if intensity.strength == 0 {
                self.entity_store.intensities.remove(&ph_id);
                evaporated_pheromones.push(ph_id.clone());
            }
        }

        for ph_id in evaporated_pheromones {
            self.entity_store.remove_position(&ph_id);
            self.entity_store.pheromones.remove(&ph_id);
        }
    }

    pub fn tick(&mut self) {
        self.ants();
        self.pheromones();
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "COMPONENTS")?;
        // writeln!(f, "----------")?;
        // writeln!(f, "positions: {:#?}", self.entity_store.positions)?;
        // writeln!(
        //     f,
        //     "positions_lookup: {:#?}",
        //     self.entity_store.positions_lookup
        // )?;
        // writeln!(
        //     f,
        //     "releasing_pheromones: {:#?}",
        //     self.entity_store.releasing_pheromones
        // )?;
        // writeln!(f, "intensities: {:#?}", self.entity_store.intensities)?;
        // writeln!(f, "----------")?;

        let integer_width = self.width.round() as u64;
        let integer_height = self.height.round() as u64;
        let separator = "|".to_owned() + &(0..integer_width).map(|_| "---|").collect::<String>();

        writeln!(f, "{}", separator)?;
        for row in 0..integer_height {
            let mut row_1 = String::new();
            let mut row_2 = String::new();
            for col in 0..integer_width {
                let mut cell_color = "white";
                let mut cell_value_row_1: String = "   ".to_string();;
                let mut cell_value_row_2: String = "   ".to_string();;
                let pos = PositionComponent {
                    x: col as f64,
                    y: row as f64,
                };

                if let Some(ids) = self.entity_store.get_entities_at(&pos) {
                    for id in ids {
                        match self.entity_store.entity_types.get(id) {
                            Some(EntityType::Ant) => {
                                cell_value_row_1 = "◆".to_string()
                                    + &cell_value_row_1
                                        [cell_value_row_1.char_indices().nth(1).unwrap().0..];
                            }
                            Some(EntityType::Sugar) => {
                                cell_value_row_1 = "■■■".to_string();
                                cell_color = "green";
                            }
                            Some(EntityType::Pheromone) => {
                                cell_value_row_2 = format!(
                                    "{:03}",
                                    self.entity_store.intensities.get(id).unwrap().strength
                                );
                            }
                            _ => {}
                        }
                    }
                }
                row_1 += &format!("|{}", cell_value_row_1.color(cell_color));
                row_2 += &format!("|{}", cell_value_row_2.color(cell_color));
            }

            writeln!(f, "{}|", row_1)?;
            writeln!(f, "{}|", row_2)?;
            writeln!(f, "{}", separator)?;
        }

        writeln!(f, "")?;

        Ok(())
    }
}
