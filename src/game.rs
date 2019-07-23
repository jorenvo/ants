use crate::components::*;
use crate::entities::*;
use crate::entity_store::*;
use crate::rand::Rng;
use colored::*;
use rand::prelude::SeedableRng;
use rand_distr::{Distribution, Normal};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::fmt;

thread_local!(static RNG: RefCell<rand::rngs::StdRng> = RefCell::new(SeedableRng::from_seed([0; 32])));

// TODO is this a System?
pub struct Game {
    width: f64,
    height: f64,
    pub entity_store: EntityStore,
}

impl Game {
    pub fn init(entity_store: EntityStore, width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            entity_store,
        }
    }

    fn pos_is_in_bounds(&self, pos: &PositionComponent) -> bool {
        pos.x >= 0.0 && pos.y >= 0.0 && pos.x < self.width && pos.y < self.height
    }

    fn pos_can_be_occupied(&self, pos: &PositionComponent) -> bool {
        if self.pos_is_in_bounds(pos) {
            !self.entity_store.pos_is_impenetrable(pos)
        } else {
            false
        }
    }

    fn calc_random_direction(&self, direction: &DirectionComponent) -> DirectionComponent {
        let std_dev = 1.0 / 3.0; // 99.7% is within 3x std dev
        let normal = Normal::new(0.0, std_dev).unwrap();
        let mut r = RNG.with(|rng| normal.sample(&mut *rng.borrow_mut()));

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
        ph_type: PheromoneType,
    ) -> Option<Vec<DirectionComponent>> {
        let mut directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let diagonals = [1, -1];
        for i in &diagonals {
            for j in &diagonals {
                directions.push((*i, *j));
            }
        }

        let positions = directions
            .iter()
            .map(|d| PositionComponent {
                x: pos.x + f64::from(d.0),
                y: pos.y + f64::from(d.1),
            })
            .filter(|p| self.pos_can_be_occupied(&p));

        let mut strength_to_dir = vec![];
        for new_pos in positions {
            if let Some(ph_id) = self
                .entity_store
                .get_pheromone_with_type_at(&new_pos, ph_type)
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

        if strength_to_dir.is_empty() {
            None
        } else {
            // desc sort
            strength_to_dir.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            Some(strength_to_dir.into_iter().map(|a| a.1).collect())
        }
    }

    fn dir_to_strongest_adjecent_pheromone(
        &self,
        ant_id: EntityIndex,
        pos: &PositionComponent,
        direction: &DirectionComponent,
        ph_type: PheromoneType,
        allow_sharp_turns: bool,
    ) -> Option<DirectionComponent> {
        if let Some(dirs) = self.dirs_to_strongest_adjecent_pheromones(pos, ph_type) {
            for dir in dirs {
                let new_angle = dir.y.atan2(dir.x).abs();
                let current_angle = direction.y.atan2(direction.x).abs();
                let angle_diff = (new_angle - current_angle).abs();

                // only allow 90° (PI / 2) turns or less
                if !allow_sharp_turns && angle_diff > PI / 1.8 {
                    continue;
                }

                let new_pos = PositionComponent {
                    x: pos.x + dir.x,
                    y: pos.y + dir.y,
                };

                if !self.entity_store.in_short_memory(ant_id, &new_pos) {
                    return Some(dir);
                }
            }
        }

        None
    }

    fn get_new_ant_direction(
        &self,
        ant_id: EntityIndex,
        pos: &PositionComponent,
        direction: &DirectionComponent,
    ) -> DirectionComponent {
        let mut direction = direction.clone();
        let is_adventurous = self.entity_store.adventurous.get(&ant_id).is_some();
        assert!(!is_adventurous);
        let allow_sharp_turns = self
            .entity_store
            .get_entities_with_type_at(pos, EntityType::Sugar)
            .is_some()
            || self
                .entity_store
                .get_entities_with_type_at(pos, EntityType::Base)
                .is_some();

        if self.entity_store.carrying_food.get(&ant_id).is_none() {
            if let Some(dir) = self.dir_to_strongest_adjecent_pheromone(
                ant_id,
                pos,
                &direction,
                PheromoneType::Food,
                allow_sharp_turns,
            ) {
                return dir;
            }
        }

        if self.entity_store.carrying_food.get(&ant_id).is_some() {
            if let Some(dir) = self.dir_to_strongest_adjecent_pheromone(
                ant_id,
                pos,
                &direction,
                PheromoneType::Base,
                allow_sharp_turns,
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

    fn handle_new_ant_pos(&mut self, ant_id: EntityIndex, new_pos: &PositionComponent) {
        let carrying_food = self.entity_store.carrying_food.get(&ant_id).is_some();
        let is_base = self
            .entity_store
            .get_entities_with_type_at(&new_pos, EntityType::Base)
            .is_some();
        let is_food = self
            .entity_store
            .get_entities_with_type_at(&new_pos, EntityType::Sugar)
            .is_some();

        if carrying_food {
            self.entity_store.releasing_pheromones.insert(
                ant_id,
                ReleasingPheromoneComponent {
                    ph_type: PheromoneType::Base,
                    ticks_left: 999, // todo
                },
            );
        } else {
            self.entity_store.releasing_pheromones.insert(
                ant_id,
                ReleasingPheromoneComponent {
                    ph_type: PheromoneType::Food,
                    ticks_left: 999, // todo
                },
            );
        }

        if carrying_food && is_base {
            println!("ant {} delivered food!", ant_id);
            self.entity_store.food_in_base += 1;
            self.entity_store.carrying_food.remove(&ant_id);
            self.entity_store.clear_memory(ant_id);
        }

        if !carrying_food && is_food {
            self.entity_store
                .carrying_food
                .insert(ant_id, CarryingFoodComponent {});
            self.entity_store.clear_memory(ant_id);
        }

        self.entity_store.add_to_short_memory(ant_id, new_pos);
    }

    fn remove_pheromone(&mut self, ph_id: EntityIndex) {
        self.entity_store.remove_position(ph_id);
        self.entity_store.intensities.remove(&ph_id);
        self.entity_store.pheromone_types.remove(&ph_id);
        self.entity_store.pheromone_generations.remove(&ph_id);
        self.entity_store.pheromones.remove(&ph_id);
    }

    fn merge_and_clear_pheromones(
        &mut self,
        pos: &PositionComponent,
        ph_type: PheromoneType,
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
            .get_entities_with_type_at(&pos, EntityType::Pheromone)
        {
            let pheromones: Vec<&EntityIndex> = pheromones
                .iter()
                .filter(|id| self.entity_store.pheromone_types.get(id).unwrap() == &ph_type)
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
        ph_type: PheromoneType,
        intensity: &IntensityComponent,
    ) -> EntityIndex {
        let (intensity, generation) =
            self.merge_and_clear_pheromones(&pos, ph_type, intensity.strength);
        let ph_id = self.entity_store.create_entity(EntityType::Pheromone);
        self.entity_store.update_position(ph_id, &pos);
        self.entity_store.intensities.insert(ph_id, intensity);
        self.entity_store.pheromone_types.insert(ph_id, ph_type);
        self.entity_store
            .pheromone_generations
            .insert(ph_id, generation);

        ph_id
    }

    fn release_pheromones(&mut self, ant_id: EntityIndex) {
        const NEW_PHEROMONE_STRENGTH: u32 = 16;

        if let Some(releasing_pheromone_comp) =
            self.entity_store.releasing_pheromones.get_mut(&ant_id)
        {
            releasing_pheromone_comp.ticks_left -= 1;

            if releasing_pheromone_comp.ticks_left == 0 {
                self.entity_store.releasing_pheromones.remove(&ant_id);
            } else {
                match releasing_pheromone_comp.ph_type {
                    PheromoneType::Food => {
                        let ant_pos = self.entity_store.get_position(ant_id).unwrap().clone();
                        let strength = if self
                            .entity_store
                            .get_entities_with_type_at(&ant_pos, EntityType::Sugar)
                            .is_some()
                        {
                            NEW_PHEROMONE_STRENGTH * 10
                        } else {
                            NEW_PHEROMONE_STRENGTH
                        };

                        self.increase_pheromone_strength_at(
                            &ant_pos,
                            PheromoneType::Food,
                            &IntensityComponent { strength },
                        );
                    }
                    PheromoneType::Base => {
                        let ant_pos = self.entity_store.get_position(ant_id).unwrap().clone();
                        let strength = if self
                            .entity_store
                            .get_entities_with_type_at(&ant_pos, EntityType::Base)
                            .is_some()
                        {
                            NEW_PHEROMONE_STRENGTH * 10
                        } else {
                            NEW_PHEROMONE_STRENGTH
                        };
                        self.increase_pheromone_strength_at(
                            &ant_pos,
                            PheromoneType::Base,
                            &IntensityComponent { strength },
                        );
                    }
                }
            }
        }
    }

    fn ants(&mut self) {
        let mut new_positions: Vec<(EntityIndex, PositionComponent)> = vec![];
        let mut new_adventurous: Vec<EntityIndex> = vec![];

        for ant_id in self.entity_store.ants.keys() {
            let pos = self.entity_store.get_position(*ant_id).unwrap();

            if self.entity_store.builders.get(&ant_id).is_some() {
                new_positions.push((*ant_id, pos.clone()));
            } else {
                let mut new_pos = PositionComponent::default();
                let direction = self
                    .entity_store
                    .get_direction(*ant_id)
                    .unwrap_or(&DirectionComponent { x: 1.0, y: 0.0 });
                let direction = self.get_new_ant_direction(*ant_id, pos, direction);
                new_pos.x = pos.x + direction.x;
                new_pos.y = pos.y + direction.y;

                // round to 0.01
                new_pos.x = (new_pos.x * 100.0).round() / 100.0;
                new_pos.y = (new_pos.y * 100.0).round() / 100.0;

                new_positions.push((*ant_id, new_pos.clone()));
            }

            // let is_adventurous: f32 = RNG.with(|rng| (*rng.borrow_mut()).gen());
            // if is_adventurous > 0.95 {
            //     new_adventurous.push(*ant_id);
            // }
        }

        for (ant_id, pos) in new_positions {
            self.entity_store.update_position(ant_id, &pos);
            self.handle_new_ant_pos(ant_id, &pos);
            self.release_pheromones(ant_id);
        }

        let mut depleted_adventurous: Vec<EntityIndex> = vec![];
        for (ant_id, adventurous) in &mut self.entity_store.adventurous {
            adventurous.ticks_left -= 1;

            if adventurous.ticks_left == 0 {
                depleted_adventurous.push(*ant_id);
            }
        }

        for depleted in depleted_adventurous {
            self.entity_store.adventurous.remove(&depleted);
        }

        for ant_id in new_adventurous {
            self.entity_store
                .adventurous
                .insert(ant_id, AdventurousComponent { ticks_left: 4 });
        }
    }

    fn pheromones(&mut self) {
        let mut to_decrement = Vec::new();
        for id in self.entity_store.intensities.keys() {
            let pos = self.entity_store.get_position(*id).unwrap();
            if self
                .entity_store
                .get_entities_with_type_at(pos, EntityType::Sugar)
                .is_some()
                || self
                    .entity_store
                    .get_entities_with_type_at(pos, EntityType::Base)
                    .is_some()
            {
                continue;
            }

            to_decrement.push(*id);
        }

        let mut to_remove = Vec::new();
        for id in to_decrement {
            let intensity = self.entity_store.intensities.get_mut(&id).unwrap();
            intensity.strength = intensity.strength.saturating_sub(1);

            if intensity.strength == 0 {
                to_remove.push(id);
            }
        }

        for id in to_remove {
            self.remove_pheromone(id);
        }
    }

    pub fn add_deneubourg_walls(&mut self) {
        let mut index;
        let mut y;
        for i in 0..5 {
            index = self.entity_store.create_entity(EntityType::Wall);
            y = if i == 0 || i == 4 { 1.5 } else { 0.5 };
            self.entity_store
                .update_position(index, &PositionComponent { x: f64::from(i), y });

            index = self.entity_store.create_entity(EntityType::Wall);
            y = if i == 0 || i == 4 { 3.5 } else { 4.5 };
            self.entity_store
                .update_position(index, &PositionComponent { x: f64::from(i), y })
        }

        // corners
        index = self.entity_store.create_entity(EntityType::Wall);
        self.entity_store
            .update_position(index, &PositionComponent { x: 0.5, y: 0.5 });
        index = self.entity_store.create_entity(EntityType::Wall);
        self.entity_store
            .update_position(index, &PositionComponent { x: 4.5, y: 0.5 });
        index = self.entity_store.create_entity(EntityType::Wall);
        self.entity_store
            .update_position(index, &PositionComponent { x: 0.5, y: 4.5 });
        index = self.entity_store.create_entity(EntityType::Wall);
        self.entity_store
            .update_position(index, &PositionComponent { x: 4.5, y: 4.5 });

        // middle
        index = self.entity_store.create_entity(EntityType::Wall);
        self.entity_store
            .update_position(index, &PositionComponent { x: 2.5, y: 2.5 });
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

        let integer_width = self.width.round() as u32;
        let integer_height = self.height.round() as u32;
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
                    x: f64::from(col),
                    y: f64::from(row),
                };

                if let Some(ids) = self.entity_store.get_entities_at(&pos) {
                    for id in ids {
                        match self.entity_store.entity_types.get(id) {
                            Some(EntityType::Ant) => {
                                cell_value_row_1 = format!("{}", id)
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

        Ok(())
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    fn assert_greater_or_equal_then<T: fmt::Display + std::cmp::PartialOrd>(a: T, b: T) {
        println!("{} >= {}", a, b);
        assert!(a >= b);
    }

    fn init_game(width: f64, height: f64, ants: usize) -> Game {
        let mut game = Game::init(EntityStore::default(), width, height);

        for i in 0..ants {
            let index = game.entity_store.create_entity(EntityType::Ant);
            game.entity_store.update_position(
                index,
                &PositionComponent {
                    x: (0.5 + i as f64) % width,
                    y: height / 2.0,
                },
            );
        }

        let index = game.entity_store.create_entity(EntityType::Base);
        game.entity_store.update_position(
            index,
            &PositionComponent {
                x: 0.5,
                y: height / 2.0,
            },
        );

        let index = game.entity_store.create_entity(EntityType::Sugar);
        game.entity_store.update_position(
            index,
            &PositionComponent {
                x: width - 0.5,
                y: height / 2.0,
            },
        );

        game
    }

    #[test]
    fn test_5x5_open() {
        let mut game = init_game(5.0, 5.0, 1);

        for _ in 0..300 {
            game.tick();
        }

        assert_greater_or_equal_then(game.entity_store.food_in_base, 32);
    }

    #[test]
    fn test_5x5_optimal_deneubourg_walls_1_ant() {
        let mut game = init_game(5.0, 5.0, 1);
        game.add_deneubourg_walls();

        for _ in 0..300 {
            game.tick();
        }

        assert_greater_or_equal_then(game.entity_store.food_in_base, 35);
    }

    #[test]
    fn test_10x10_open() {
        // let mut game = init_game(10.0, 10.0, 10);

        // for _ in 0..300 {
        //     game.tick();
        // }

        // assert_greater_or_equal_then(game.entity_store.food_in_base, 150);
    }
}
