use crate::components::*;
use crate::entities::*;
use std::collections::BTreeMap;
use std::fmt;

type EntityIndex = usize;

#[derive(Default)]
pub struct EntityStore {
    pub new_index: EntityIndex,
    pub types: BTreeMap<EntityIndex, EntityType>,
    pub positions: BTreeMap<EntityIndex, PositionComponent>,
    pub edibles: BTreeMap<EntityIndex, EdibleComponent>,
    // TODO: reverse_positions
    pub ants: BTreeMap<EntityIndex, AntEntity>,
    pub pheromones: BTreeMap<EntityIndex, PheromoneEntity>,
    pub sugars: BTreeMap<EntityIndex, SugarEntity>,
}

impl EntityStore {
    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    pub fn create_entity(&mut self, entity_type: &EntityType) -> EntityIndex {
        let index = self.get_new_index();
        match entity_type {
            EntityType::Ant => {
                self.positions.insert(index, PositionComponent::default());
                self.ants.insert(index, AntEntity {});
            }
            EntityType::Pheromone => {
                self.positions.insert(index, PositionComponent::default());
                self.pheromones.insert(index, PheromoneEntity {});
            }
            EntityType::Sugar => {
                self.positions
                    .insert(index, PositionComponent { x: 10, y: 10 });
                self.edibles.insert(index, EdibleComponent::default());
                self.sugars.insert(index, SugarEntity {});
            }
        }
        self.types.insert(index, entity_type.clone());

        index
    }

    pub fn get_pheromone_at(&self, search_pos: &PositionComponent) -> Option<EntityIndex> {
        // TODO do this more efficiently
        // TODO this should probably return Vec<EntityIndex>
        for (id, _) in &self.pheromones {
            if let Some(pos) = self.positions.get(id) {
                if pos == search_pos {
                    return Some(*id);
                }
            }
        }

        None
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
            } else if self.sugars.get(&i).is_some() {
                if let Some(pos) = self.positions.get(&i) {
                    writeln!(f, "sugar {} at {}, {}", i, pos.x, pos.y)?;
                } else {
                    writeln!(f, "sugar {} has no position!", i)?;
                }
            } else {
                writeln!(f, "unknown entity {}!", i)?;
            }
        }

        Ok(())
    }
}