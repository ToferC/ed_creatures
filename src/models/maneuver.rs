use uuid::Uuid;
use chrono::NaiveDateTime;


#[derive(Debug)]
pub struct Maneuver {
    uuid: Uuid,
    creature_id: Uuid,
    name: String,
    action_type: String,
    target: Option<String>,
    action_step: u32,
    effect_step: u32,
    details: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}