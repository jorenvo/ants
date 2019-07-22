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
}

impl Game {
    pub fn init(entity_store: EntityStore, width: f64, height: f64) -> Self {
        Self {
            width: width,
            height: height,
            entity_store: entity_store,
        }
    }

    fn pos_is_in_bounds(&self, pos: &PositionComponent) -> bool {
        pos.x >= 0.0 && pos.y >= 0.0 && pos.x < self.width && pos.y < self.height
    }

    fn pos_can_be_occupied(&self, pos: &PositionComponent) -> bool {
        if !self.pos_is_in_bounds(pos) {
            false
        } else {
            !self.entity_store.pos_is_impenetrable(pos)
        }
    }

    fn calc_random_direction(&self, direction: &DirectionComponent) -> DirectionComponent {
        static mut CALL_COUNT: u8 = 0;
        unsafe {
            CALL_COUNT = CALL_COUNT.wrapping_add(1);
        }

        let std_dev = 1.0 / 3.0; // 99.7% is within 3x std dev
        let normal = Normal::new(0.0, std_dev).unwrap();

        let seed = unsafe {
            [
                CALL_COUNT,
                (direction.x as u64 % 256) as u8,
                (direction.y as u64 % 256) as u8,
                (self.entity_store.new_index % 256) as u8,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]
        };
        let mut rng: rand::rngs::StdRng = SeedableRng::from_seed(seed);
        let mut r = normal.sample(&mut rng);

        r *= PI; // [-pi, pi], centered around pi
        r += direction.y.atan2(direction.x);

        DirectionComponent {
            x: r.cos(),
            y: r.sin(),
        }
    }

    fn dirs_to_strongest_adjecent_pheromones(
        &self,
        pos: &PositionComponent,
        ph_type: &PheromoneType,
    ) -> Option<Vec<DirectionComponent>> {
        let mut directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let diagonals = [1, -1];
        for i in diagonals.iter() {
            for j in diagonals.iter() {
                directions.push((*i, *j));
            }
        }

        let positions = directions
            .iter()
            .map(|d| PositionComponent {
                x: pos.x + d.0 as f64,
                y: pos.y + d.1 as f64,
            })
            .filter(|p| self.pos_can_be_occupied(&p));

        let mut strength_to_dir = vec![];
        for new_pos in positions {
            if let Some(ph_id) = self
                .entity_store
                .get_pheromone_with_type_at(&new_pos, &ph_type)
            {
                let intensity = self.entity_store.intensities.get(&ph_id).unwrap();
                strength_to_dir.push((
                    intensity.strength,
                    DirectionComponent {
                        x: new_pos.x - pos.x,
                        y: new_pos.y - pos.y,
                    },
                ));
            }
        }

        if strength_to_dir.len() >= 1 {
            // desc sort
            strength_to_dir.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            Some(strength_to_dir.into_iter().map(|a| a.1).collect())
        } else {
            None
        }
    }

    fn dir_to_strongest_adjecent_pheromone(
        &self,
        ant_id: &EntityIndex,
        pos: &PositionComponent,
        direction: &DirectionComponent,
        ph_type: &PheromoneType,
    ) -> Option<DirectionComponent> {
        if let Some(dirs) = self.dirs_to_strongest_adjecent_pheromones(pos, ph_type) {
            for dir in dirs {
                let new_angle = dir.y.atan2(dir.x).abs();
                let current_angle = direction.y.atan2(direction.x).abs();
                let angle_diff = (new_angle - current_angle).abs();
                println!(
                    "({}, {}) (angle: {}), ({}, {}) (angle: {}) = {}",
                    direction.x, direction.y, current_angle, dir.x, dir.y, new_angle, angle_diff
                );

                // only allow 90° (PI / 2) turns or less
                if angle_diff > PI / 1.8 {
                    continue;
                }

                let new_pos = PositionComponent {
                    x: pos.x + dir.x,
                    y: pos.y + dir.y,
                };
                if !self.entity_store.in_short_memory(ant_id, &new_pos) {
                    println!(
                        "going to pheromone at {:?} at angle {} ({:?} -> {:?})",
                        new_pos, angle_diff, direction, dir
                    );
                    return Some(dir);
                }
            }
        }

        None
    }

    fn get_new_ant_direction(
        &self,
        ant_id: &EntityIndex,
        pos: &PositionComponent,
        direction: &DirectionComponent,
    ) -> DirectionComponent {
        let mut direction = direction.clone();

        if self.entity_store.carrying_food.get(&ant_id).is_none() {
            if let Some(dir) = self.dir_to_strongest_adjecent_pheromone(
                ant_id,
                pos,
                &direction,
                &PheromoneType::Food,
            ) {
                return dir;
            }
        }

        if self.entity_store.carrying_food.get(&ant_id).is_some() {
            if let Some(dir) = self.dir_to_strongest_adjecent_pheromone(
                ant_id,
                pos,
                &direction,
                &PheromoneType::Base,
            ) {
                return dir;
            }
        }

        let mut dir = self.calc_random_direction(&direction);
        let mut new_pos = PositionComponent {
            x: pos.x + dir.x,
            y: pos.y + dir.y,
        };
        let mut tries = 1;
        while !self.pos_can_be_occupied(&new_pos)
            || (self.entity_store.in_short_memory(ant_id, &new_pos) && tries < 8)
        {
            if tries == 8 {
                direction.x = -direction.x;
                direction.y = -direction.y;
            }
            dir = self.calc_random_direction(&direction);
            new_pos = PositionComponent {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
            };
            tries += 1;
        }

        dir
    }

    fn handle_new_ant_pos(&mut self, ant_id: &EntityIndex, new_pos: &PositionComponent) {
        let carrying_food = self.entity_store.carrying_food.get(&ant_id).is_some();
        let is_base = self
            .entity_store
            .get_entities_with_type_at(&new_pos, &EntityType::Base)
            .is_some();
        let is_food = self
            .entity_store
            .get_entities_with_type_at(&new_pos, &EntityType::Sugar)
            .is_some();

        if carrying_food {
            self.entity_store.releasing_pheromones.insert(
                *ant_id,
                ReleasingPheromoneComponent {
                    ph_type: PheromoneType::Base,
                    ticks_left: 999, // todo
                },
            );
        } else {
            self.entity_store.releasing_pheromones.insert(
                *ant_id,
                ReleasingPheromoneComponent {
                    ph_type: PheromoneType::Food,
                    ticks_left: 999, // todo
                },
            );
        }

        if carrying_food && is_base {
            println!("ant {} delivered food!", ant_id);
            self.entity_store.carrying_food.remove(ant_id);
            self.entity_store.clear_memory(ant_id);
        }

        if !carrying_food && is_food {
            self.entity_store
                .carrying_food
                .insert(*ant_id, CarryingFoodComponent {});
            self.entity_store.clear_memory(ant_id);
        }

        self.entity_store.add_to_short_memory(ant_id, new_pos);
    }

    fn remove_pheromone(&mut self, ph_id: EntityIndex) {
        self.entity_store.remove_position(&ph_id);
        self.entity_store.intensities.remove(&ph_id);
        self.entity_store.pheromone_types.remove(&ph_id);
        self.entity_store.pheromone_generations.remove(&ph_id);
        self.entity_store.pheromones.remove(&ph_id);
    }

    fn merge_and_clear_pheromones(
        &mut self,
        pos: &PositionComponent,
        ph_type: &PheromoneType,
        extra_strength: u32,
    ) -> (IntensityComponent, PheromoneGenerationComponent) {
        let mut intensity = IntensityComponent {
            strength: extra_strength,
        };

        // When merging pheromones the oldest generation should be
        // kept. This allows ants to reinforce existing pheromones.
        let mut generation = PheromoneGenerationComponent {
            generation: self.entity_store.pheromone_generation,
        };
        let mut pheromones_to_delete = vec![];

        if let Some(pheromones) = self
            .entity_store
            .get_entities_with_type_at(&pos, &EntityType::Pheromone)
        {
            let pheromones: Vec<&EntityIndex> = pheromones
                .iter()
                .filter(|id| self.entity_store.pheromone_types.get(id).unwrap() == ph_type)
                .collect();

            if !pheromones.is_empty() {
                let merged_strength: u32 = pheromones
                    .iter()
                    .map(|p| self.entity_store.intensities.get(p).unwrap().strength)
                    .sum();
                intensity.strength += merged_strength;

                generation.generation = pheromones
                    .iter()
                    .map(|p| {
                        self.entity_store
                            .pheromone_generations
                            .get(p)
                            .unwrap()
                            .generation
                    })
                    .min()
                    .unwrap_or(self.entity_store.pheromone_generation);

                pheromones_to_delete = pheromones.iter().map(|p| **p).collect();
            }
        }

        for ph in pheromones_to_delete {
            self.remove_pheromone(ph);
        }

        (intensity, generation)
    }

    fn increase_pheromone_strength_at(
        &mut self,
        pos: &PositionComponent,
        ph_type: &PheromoneType,
        intensity: &IntensityComponent,
    ) -> EntityIndex {
        let (intensity, generation) =
            self.merge_and_clear_pheromones(&pos, ph_type, intensity.strength);
        let ph_id = self.entity_store.create_entity(&EntityType::Pheromone);
        self.entity_store.update_position(&ph_id, &pos);
        self.entity_store.intensities.insert(ph_id, intensity);
        self.entity_store.pheromone_types.insert(ph_id, *ph_type);
        self.entity_store
            .pheromone_generations
            .insert(ph_id, generation);

        ph_id
    }

    fn release_pheromones(&mut self, ant_id: &EntityIndex) {
        const NEW_PHEROMONE_STRENGTH: u32 = 16;

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
                        let strength = if self
                            .entity_store
                            .get_entities_with_type_at(&ant_pos, &EntityType::Sugar)
                            .is_some()
                        {
                            NEW_PHEROMONE_STRENGTH * 10
                        } else {
                            NEW_PHEROMONE_STRENGTH
                        };

                        self.increase_pheromone_strength_at(
                            &ant_pos,
                            &PheromoneType::Food,
                            &IntensityComponent { strength: strength },
                        );
                    }
                    PheromoneType::Base => {
                        let ant_pos = self.entity_store.get_position(ant_id).unwrap().clone();
                        let strength = if self
                            .entity_store
                            .get_entities_with_type_at(&ant_pos, &EntityType::Base)
                            .is_some()
                        {
                            NEW_PHEROMONE_STRENGTH * 10
                        } else {
                            NEW_PHEROMONE_STRENGTH
                        };
                        self.increase_pheromone_strength_at(
                            &ant_pos,
                            &PheromoneType::Base,
                            &IntensityComponent { strength: strength },
                        );
                    }
                }
            }
        }
    }

    fn ants(&mut self) {
        let mut new_positions: Vec<(EntityIndex, PositionComponent)> = vec![];
        for (ant_id, _) in &self.entity_store.ants {
            let pos = self.entity_store.get_position(ant_id).unwrap();

            if self.entity_store.builders.get(&ant_id).is_some() {
                new_positions.push((*ant_id, pos.clone()));
            } else {
                let mut new_pos = PositionComponent::default();
                let direction = self
                    .entity_store
                    .get_direction(ant_id)
                    .unwrap_or(&DirectionComponent { x: 1.0, y: 0.0 });
                let direction = self.get_new_ant_direction(ant_id, pos, direction);
                new_pos.x = pos.x + direction.x;
                new_pos.y = pos.y + direction.y;

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
        let mut to_remove = Vec::new();

        for (id, intensity) in self.entity_store.intensities.iter_mut() {
            intensity.strength -= 1;

            if intensity.strength == 0 {
                to_remove.push(*id);
            }
        }

        for id in to_remove {
            self.remove_pheromone(id);
        }
    }

    pub fn tick(&mut self) {
        self.pheromones();
        self.ants();
        self.entity_store.pheromone_generation += 1;
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
        let separator = "|".to_owned()
            + &(0..integer_width)
                .map(|_| "-----------|")
                .collect::<String>();

        writeln!(f, "{}", separator)?;
        for row in 0..integer_height {
            let mut row_1 = String::new();
            let mut row_2 = String::new();
            let mut row_3 = String::new();
            for col in 0..integer_width {
                let mut cell_color = "white";
                let mut cell_value_row_1: String = "           ".to_string();
                let mut cell_value_row_2: String = "           ".to_string();
                let mut cell_value_row_3: String = "           ".to_string();
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

                                cell_color = "red";
                                if self.entity_store.carrying_food.get(&id).is_some() {
                                    cell_color = "yellow";
                                }
                            }
                            Some(EntityType::Sugar) => {
                                cell_value_row_1 =
                                    cell_value_row_1.chars().next().unwrap_or(' ').to_string()
                                        + &"■■■■■■■■■■".to_string();
                                cell_color = "green";
                            }
                            Some(EntityType::Base) => {
                                cell_value_row_1 =
                                    cell_value_row_1.chars().next().unwrap_or(' ').to_string()
                                        + &"■■■■■■■■■■".to_string();
                                cell_color = "blue";
                            }
                            Some(EntityType::Pheromone) => {
                                match self.entity_store.pheromone_types.get(&id).unwrap() {
                                    PheromoneType::Food => {
                                        cell_value_row_2 = format!(
                                            "{:7}|{:3}",
                                            self.entity_store.intensities.get(id).unwrap().strength,
                                            self.entity_store
                                                .pheromone_generations
                                                .get(id)
                                                .unwrap()
                                                .generation,
                                        );
                                    }
                                    PheromoneType::Base => {
                                        cell_value_row_3 = format!(
                                            "{:7}|{:3}",
                                            self.entity_store.intensities.get(id).unwrap().strength,
                                            self.entity_store
                                                .pheromone_generations
                                                .get(id)
                                                .unwrap()
                                                .generation,
                                        );
                                    }
                                }
                            }
                            Some(EntityType::Wall) => {
                                cell_value_row_1 =
                                    cell_value_row_1.chars().next().unwrap_or(' ').to_string()
                                        + &"xxxxxxxxxx".to_string();
                                cell_color = "red";
                            }
                            None => {}
                        }
                    }
                }
                row_1 += &format!("|{}", cell_value_row_1.color(cell_color));
                row_2 += &format!("|{}", cell_value_row_2.color(cell_color));
                row_3 += &format!("|{}", cell_value_row_3.color(cell_color));
            }

            writeln!(f, "{}|", row_1)?;
            writeln!(f, "{}|", row_2)?;
            writeln!(f, "{}|", row_3)?;
            writeln!(f, "{}", separator)?;
        }

        writeln!(f, "")?;

        Ok(())
    }
}
