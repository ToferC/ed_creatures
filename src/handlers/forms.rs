use serde::Deserialize;
use crate::models::Rarity;


#[derive(Deserialize, Debug)]
pub struct DeleteForm {
    pub verify: String,
}

#[derive(Deserialize, Debug)]
pub struct CreatureForm {
    pub name: String,
    pub jungle: String,
    pub desert: String,
    pub forest: String,
    pub plains: String,
    pub urban: String,
    pub mountain: String,
    pub cavern: String,
    pub kaer: String,
    pub any: String,
    pub rarity: Rarity,
    pub circle_rank: i32,
    pub dexterity: i32,
    pub strength: i32,
    pub constitution: i32,
    pub perception: i32,
    pub willpower: i32,
    pub charisma: i32,
    pub initiative: i32,
    pub pd: i32,
    pub md: i32,
    pub sd: i32,
    pub pa: i32,
    pub ma: i32,
    pub unconsciousness_rating: i32,
    pub death_rating: i32,
    pub wound: i32,
    pub knockdown: i32,
    pub actions: i32,
    pub movement: String,
    pub recovery_rolls: i32,
    pub karma: i32,
    pub image_url: Option<String>,
}