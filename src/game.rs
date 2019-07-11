use crate::components::*;
use crate::entities::*;
use crate::entity_store::*;
use colored::*;
use rand::prelude::SeedableRng;
use rand_distr::{Distribution, Normal};
use std::f64::consts::PI;
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

    fn calc_random_direction(&self, direction: &DirectionComponent) -> DirectionComponent {
        let std_dev = 1.0 / 3.0; // 99.7% is within 3x std dev
        let normal = Normal::new(0.0, std_dev).unwrap();
        let mut r = normal.sample(&mut rand::thread_rng());

        r *= PI; // [-pi, pi], centered around pi
        r += direction.y.atan2(direction.x);

        DirectionComponent {
            x: r.cos(),
            y: r.sin(),
        }
    }

    fn get_random_direction(
        &self,
        pos: &PositionComponent,
        direction: &DirectionComponent,
    ) -> DirectionComponent {
        let mut dir = self.calc_random_direction(direction);

        while pos.x + dir.x < 0.0
            || pos.y + dir.y < 0.0
            || pos.x + dir.x >= self.width
            || pos.y + dir.y >= self.height
        {
            dir = self.calc_random_direction(direction);
        }

        dir
    }

    fn handle_new_ant_pos(&mut self, ant_id: &EntityIndex, new_pos: &PositionComponent) {
        let entities_at_new_pos = self.entity_store.get_entities_at(&new_pos);
        let mut new_releasing_ph_components: Vec<(EntityIndex, ReleasingPheromoneComponent)> =
            vec![];
        let mut new_sensed_pheromones: Vec<(EntityIndex, EntityIndex)> = vec![];

        if let Some(entities_at_new_pos) = entities_at_new_pos {
            for id in entities_at_new_pos {
                match self.entity_store.entity_types.get(id) {
                    Some(EntityType::Pheromone) => {
                        new_sensed_pheromones.push((*ant_id, *id));
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

        self.entity_store.sensed_pheromones.clear();
        for (id, ph_id) in new_sensed_pheromones {
            self.entity_store
                .sensed_pheromones
                .insert(id, SensedPheromoneComponent { id: ph_id });
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

    fn increase_pheromone_strength_at(
        &mut self,
        pos: &PositionComponent,
        intensity: &IntensityComponent,
    ) -> EntityIndex {
        let intensity = self.merge_and_clear_pheromones(&pos, intensity.strength);
        let ph_id = self.entity_store.create_entity(&EntityType::Pheromone);
        self.entity_store.update_position(&ph_id, &pos);
        self.entity_store.intensities.insert(ph_id, intensity);

        ph_id
    }

    fn release_pheromones(&mut self, ant_id: &EntityIndex) {
        const NEW_PHEROMONE_STRENGTH: u32 = 64;

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
                        self.increase_pheromone_strength_at(
                            &ant_pos,
                            &IntensityComponent {
                                strength: NEW_PHEROMONE_STRENGTH,
                            },
                        );
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
                let random_direction = self.get_random_direction(pos, direction);
                new_pos.x = pos.x + random_direction.x;
                new_pos.y = pos.y + random_direction.y;

                // round to 0.01
                new_pos.x = (new_pos.x * 100.0).round() / 100.0;
                new_pos.y = (new_pos.y * 100.0).round() / 100.0;

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
        let mut new_pheromones: Vec<(PositionComponent, IntensityComponent)> = vec![];
        let mut evaporated_pheromones = vec![];

        for (ph_id, _) in &self.entity_store.pheromones {
            let intensity = self.entity_store.intensities.get_mut(&ph_id).unwrap();
            let strength_to_spread = (intensity.strength as f64 * 0.25).ceil() as u32;
            intensity.strength -= strength_to_spread;

            if intensity.strength == 0 {
                self.entity_store.intensities.remove(&ph_id);
                evaporated_pheromones.push(ph_id.clone());
            }

            if strength_to_spread >= 8 {
                let pos = self.entity_store.get_position(ph_id).unwrap();
                for i in 0..8 {
                    let angle = PI / 4.0 * i as f64;
                    new_pheromones.push((
                        PositionComponent {
                            x: pos.x + angle.cos(),
                            y: pos.y + angle.sin(),
                        },
                        IntensityComponent {
                            strength: strength_to_spread / 8,
                        },
                    ));
                }
            }
        }

        for ph_id in evaporated_pheromones {
            self.entity_store.remove_position(&ph_id);
            self.entity_store.pheromones.remove(&ph_id);
        }

        for (pos, intensity) in new_pheromones {
            self.increase_pheromone_strength_at(&pos, &intensity);
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
