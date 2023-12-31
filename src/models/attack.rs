use uuid::Uuid;
use chrono::NaiveDateTime;


#[derive(Debug)]
pub struct Attack {
    uuid: Uuid,
    creator_id: Uuid,
    creature_id: Uuid,
    name: String,
    attack_step: u32,
    damage_step: u32,
    details: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}