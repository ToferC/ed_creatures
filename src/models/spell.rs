use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::schema::spells;
use crate::database::connection;
use diesel_derive_enum::DbEnum;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};

use crate::models::Creature;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Associations, Identifiable, Clone)]
#[diesel(table_name = spells)]
#[belongs_to(Creature)]
pub struct Spell {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub threads: i32,
    pub weaving: i32,
    pub reattune: i32,
    pub fixed_target: i32,
    pub action_type: ActionType,
    pub target: ActionTarget,
    pub resisted_by: ResistedBy,
    pub action_step: i32,
    pub effect_step: Option<i32>,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ActionTypes"]
pub enum ActionType {
    Free,
    Simple,
    Standard,
    Move,
    Ritual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ActionTargets"]
pub enum ActionTarget {
    MysticDefense,
    PhysicalDefense,
    SocialDefense,
    Other,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ResistedBys"]
pub enum ResistedBy {
    Mystic,
    Physical,
    Other,
    NotApplicable,
}


impl Spell {
    pub fn create(spell_data: &InsertableSpell) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(spells::table)
            .values(spell_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(spell: &InsertableSpell) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = spells::table
            .filter(spells::name.eq(&spell.name))
            .distinct()
            .first(&mut conn);

        let spell = match res {
            Ok(c) => c,
            Err(e) => {
                // spell not found
                println!("{:?}", e);
                let c = Spell::create(spell).expect("Unable to create spell");
                c
            }
        };
        Ok(spell)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = spells::table
            .filter(spells::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = spells::table
            .filter(spells::name.ilike(format!("%{}%", name)))
            .load::<Spell>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_creature_id(creature_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res: Vec<Self> = spells::table
            .filter(spells::creature_id.eq(creature_id))
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(spells::table)
            .filter(spells::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;

        let res = diesel::delete(spells::table.filter(
                spells::id.eq(id)))
            .execute(&mut conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = spells)]
pub struct InsertableSpell {
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub action_type: ActionType,
    pub target: ActionTarget,
    pub resisted_by: ResistedBy,
    pub action_step: i32,
    pub effect_step: Option<i32>,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableSpell {

    pub fn default(creator_id: Uuid, creature_id: Uuid) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableSpell {
            creator_id,
            creature_id,
            name: "Generic Spell".to_owned(),
            action_type: ActionType::Standard,
            target: ActionTarget::PhysicalDefense,
            resisted_by: ResistedBy::Physical,
            action_step: 9,
            effect_step: Some(9),
            details: Some("A basic spell".to_owned()),
            created_at: today,
            updated_at: today,
        }
    }

    pub fn new(
        creator_id: Uuid,
        creature_id: Uuid,
        name: String,
        action_type: ActionType,
        target: ActionTarget,
        resisted_by: ResistedBy,
        action_step: i32,
        effect_step: Option<i32>,
        details: Option<String>,
    ) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableSpell {
            creator_id,
            creature_id,
            name,
            action_type,
            target,
            resisted_by,
            action_step,
            effect_step,
            details,
            created_at: today,
            updated_at: today,
        }
    }
}

