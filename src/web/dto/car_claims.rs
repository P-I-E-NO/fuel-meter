use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CarClaims {
    pub car_name: String,
    pub car_id: String,
    pub tank_size: i32,
    pub consumption: f32,
    pub owner: String,
}