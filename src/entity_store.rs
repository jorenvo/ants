#![deny(clippy::pedantic)]
use crate::components::*;
use crate::entities::*;
use std::collections::HashMap;
use std::fmt;

type EntityIndex = usize;

pub struct EntityStore {
    pub new_index: EntityIndex,
    pub types: HashMap<EntityIndex, EntityType>,
    pub positions: HashMap<EntityIndex, PositionComponent>,
    // TODO: reverse_positions
    pub ants: HashMap<EntityIndex, AntEntity>,
    pub pheromones: HashMap<EntityIndex, PheromoneEntity>,
}

impl EntityStore {
    pub fn init() -> Self {
        Self {
            new_index: 0,
            types: HashMap::new(),
            positions: HashMap::new(),
            ants: HashMap::new(),
            pheromones: HashMap::new(),
        }
    }

    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    pub fn create_entity(&mut self, entity_type: &EntityType) -> EntityIndex {
        let index = self.get_new_index();
        match entity_type {
            EntityType::Ant => {
                self.positions
                    .insert(index, PositionComponent { x: 0, y: 0 });
                self.ants.insert(index, AntEntity {});
            }
            EntityType::Pheromone => {
                self.positions
                    .insert(index, PositionComponent { x: 0, y: 0 });
                self.pheromones.insert(index, PheromoneEntity {});
            }
        }
        self.types.insert(index, entity_type.clone());

        index
    }
}

impl fmt::Display for EntityStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.new_index {
            if self.ants.get(&i).is_some() {
                if let Some(pos) = self.positions.get(&i) {
                    writeln!(f, "ant {} at {}, {}", i, pos.x, pos.y)?;
                } else {
                    writeln!(f, "ant {} has no position!", i)?;
                }
            } else if self.pheromones.get(&i).is_some() {
                if let Some(pos) = self.positions.get(&i) {
                    writeln!(f, "pheromone {} at {}, {}", i, pos.x, pos.y)?;
                } else {
                    writeln!(f, "pheromone {} has no position!", i)?;
                }
            } else {
                writeln!(f, "unknown entity {}!", i)?;
            }
        }

        Ok(())
    }
}