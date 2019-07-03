#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EntityType {
    Ant,
    Pheromone,
    Sugar,
}

pub struct AntEntity {}
pub struct SugarEntity {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PheromoneType {
    Food,
}

#[derive(Debug)]
pub struct PheromoneEntity {}
