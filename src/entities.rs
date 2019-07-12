#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EntityType {
    Ant,
    Pheromone,
    Base,
    Sugar,
}

pub struct AntEntity {}
pub struct SugarEntity {}
pub struct BaseEntity {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PheromoneType {
    Base,
    Food,
}

#[derive(Debug)]
pub struct PheromoneEntity {}
