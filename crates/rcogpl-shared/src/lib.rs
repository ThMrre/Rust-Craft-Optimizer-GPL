use std::collections::HashMap;

pub type ResourceSet = HashMap<Resource, f64>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    pub id: String,
}

impl Resource {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
        }
    }
}