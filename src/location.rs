use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub current_country_id: i64,
    pub locations: Vec<Loc>,
    pub wp_total: i64,
    pub total: i64,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub opened: bool,
    pub mask: Requirement,
    pub towel: String,
    pub fountain: String,
    pub locker_room: String,
    pub schedules: Vec<Schedule>,
}

#[derive(Serialize, Deserialize)]
pub enum Requirement {
    #[serde(rename = "recommended")]
    Recommended,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "not_allowed")]
    NotAllowed,
}

#[derive(Serialize, Deserialize)]
pub struct LocationSmaller {
    pub id: i64,
    pub title: String,
    pub street: String,
    pub region: String,
    pub city_name: String,
    pub state_name: String,
    pub uf: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Loc {
    Location(Location),
    LocationSmaller(LocationSmaller),
}

#[derive(Serialize, Deserialize)]
pub struct Schedule {
    pub weekdays: String,
    pub hour: String,
}
