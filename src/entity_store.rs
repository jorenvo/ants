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
}

impl EntityStore {
    pub fn init() -> Self {
        Self {
            new_index: 0,
            types: HashMap::new(),
            positions: HashMap::new(),
            ants: HashMap::new(),
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
            _ => {}
        }
        self.types.insert(index, entity_type.clone());

        index
    }
}

impl fmt::Display for EntityStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- ANTS ---")?;
        for ant_id in self.ants.keys() {
            if let Some(pos) = self.positions.get(ant_id) {
                writeln!(f, "id {} at {}, {}", ant_id, pos.x, pos.y)?;
            } else {
                writeln!(f, "id {} has no position!", ant_id)?;
            }
        }

        Ok(())
    }
}