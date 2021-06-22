use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub type_primary: Option<String>,
    pub type_secondary: Option<String>,
    pub ability_primary: Option<String>,
    pub ability_secondary: Option<String>,
    pub ability_hidden: Option<String>,
    pub hp: i64,
    pub attack: i64,
    pub defense: i64,
    pub special_attack: i64,
    pub special_defense: i64,
    pub speed: i64,
    pub image: String,
}
