use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plant {
    pub name: String,
    pub description: String,
    pub image: Option<Vec<u8>>,
    pub scientific_name: Option<String>,
    pub care_instructions: Option<String>,
    pub watering_frequency: Option<String>,
    pub maintenance_record: Option<Maintenance>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Maintenance {
    pub timestamp: NaiveDateTime,
    pub last_watered: Option<NaiveDateTime>,
    pub last_fertilized: Option<NaiveDateTime>,
    pub fertilization_type: Option<String>,
    pub image: Option<Vec<u8>>,
}

pub struct PlantResponse {
    pub name: String,
    pub description: String,
    pub image: Option<Vec<u8>>,
    pub scientific_name: Option<String>,
    pub care_instructions: Option<String>,
    pub watering_frequency: Option<String>,
    pub maintenance_record: Option<Maintenance>,
}
