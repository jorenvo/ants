use crate::entities::*;
use crate::utils::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Default)]
pub struct PositionComponent {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for PositionComponent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PositionComponent {}

impl PartialOrd for PositionComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PositionComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering_y = cmp_float(self.y, other.y);

        if ordering_y == Ordering::Equal {
            cmp_float(self.x, other.x)
        } else {
            ordering_y
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct EdibleComponent {}

#[derive(PartialEq, Debug)]
pub struct PheromoneTypeComponent {
    pub ph_type: PheromoneType,
}

#[derive(PartialEq, Debug, Default)]
pub struct IntensityComponent {
    pub strength: u8,
}

#[derive(PartialEq, Debug)]
pub struct ReleasingPheromoneComponent {
    pub ticks_left: u32,
    pub ph_type: PheromoneType,
}

#[derive(PartialEq, Debug)]
pub struct DirectionComponent {
    pub x: f64,
    pub y: f64,
}
