use rcogpl_shared::{Resource, ResourceSet};

pub struct Recipe {
    pub inputs: ResourceSet,
    pub outputs: ResourceSet,
    pub time: Option<f32>,
}

pub struct Solver;

impl Solver {
    pub fn new() -> Self {
        Self
    }

    pub fn solve(
        &self,
        _recipes: &[Recipe],
        _available: &ResourceSet,
        _goal: &ResourceSet,
    ) -> Option<Vec<Recipe>> {
        // TODO: implement solver
        None
    }
}