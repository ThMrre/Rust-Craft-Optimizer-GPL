use rcogpl_shared::{Resource, ResourceSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UniversalGameData {
    pub resources: Vec<UniversalResource>,
    pub recipes: Vec<UniversalRecipe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniversalResource {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub resource_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniversalRecipe {
    pub id: String,
    pub name: String,
    pub inputs: Vec<UniversalIngredient>,
    pub outputs: Vec<UniversalIngredient>,
    pub time: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniversalIngredient {
    pub resource: String,
    pub amount: f64,
}

pub fn load_from_json(_json: &str) -> Result<UniversalGameData, anyhow::Error> {
    // TODO: implement parsing
    unimplemented!()
}