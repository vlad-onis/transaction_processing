use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub client: i32,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}
