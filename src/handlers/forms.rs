use serde::Deserialize;
use crate::models::{Rarity, ActionType, ActionTarget, ResistedBy};


#[derive(Deserialize, Debug)]
pub struct DeleteForm {
    pub verify: String,
}

#[derive(Deserialize, Debug)]
pub struct CreatureForm {
    pub name: String,
    pub rarity: Rarity,
    pub jungle: Option<String>,
    pub desert: Option<String>,
    pub forest: Option<String>,
    pub plains: Option<String>,
    pub urban: Option<String>,
    pub mountain: Option<String>,
    pub cavern: Option<String>,
    pub swamp: Option<String>,
    pub kaer: Option<String>,
    pub any: Option<String>,
    pub circle_rank: i32,
    pub description: String,
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

#[derive(Debug, Deserialize)]
pub struct AttackForm {
    pub name: String,
    pub action_step: i32,
    pub effect_step: i32,
    pub details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PowerForm {
    pub name: String,
    pub action_type: ActionType,
    pub target: ActionTarget,
    pub resisted_by: ResistedBy,
    pub action_step: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_step: Option<i32>,
    pub details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchForm {
    pub search: String,
}

#[derive(Debug, Deserialize)]
pub struct ManeuverForm {
    pub name: String,
    pub source: String,
    pub details: String,
}