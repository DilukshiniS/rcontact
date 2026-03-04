use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
}