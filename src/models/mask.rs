use chrono::NaiveDateTime;
use uuid::Uuid;


#[derive(Debug)]
pub struct Mask {
    pub uuid: Uuid,
    pub name: String,
    pub circle: i32,
    pub dex: i32,
    pub str: i32,
    pub con: i32,
    pub per: i32,
    pub wil: i32,
    pub cha: i32,
    pub initiative: i32,
    pub pd: i32,
    pub md: i32,
    pub sd: i32,
    pub pa: i32,
    pub ma: i32,
    pub unconscious_rating: i32,
    pub death_rating: i32,
    pub wound: i32,
    pub knockdown: i32,
    pub actions: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}