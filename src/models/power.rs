use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::schema::powers;
use crate::database::connection;
use diesel_derive_enum::DbEnum;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};

use crate::models::Creature;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Associations, Identifiable, Clone)]
#[diesel(table_name = powers)]
#[belongs_to(Creature)]
pub struct Power {
    pub id: Uuid,
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


impl Power {
    pub fn create(power_data: &InsertablePower) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(powers::table)
            .values(power_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(power: &InsertablePower) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = powers::table
            .filter(powers::name.eq(&power.name))
            .distinct()
            .first(&mut conn);

        let power = match res {
            Ok(c) => c,
            Err(e) => {
                // power not found
                println!("{:?}", e);
                let c = Power::create(power).expect("Unable to create power");
                c
            }
        };
        Ok(power)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = powers::table
            .filter(powers::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = powers::table
            .filter(powers::name.ilike(format!("%{}%", name)))
            .load::<Power>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_creature_id(creature_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res: Vec<Self> = powers::table
            .filter(powers::creature_id.eq(creature_id))
            .load::<Self>(&mut conn)?;

        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(powers::table)
            .filter(powers::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;

        let res = diesel::delete(powers::table.filter(
                powers::id.eq(id)))
            .execute(&mut conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = powers)]
pub struct InsertablePower {
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

impl InsertablePower {

    pub fn default(creator_id: Uuid, creature_id: Uuid) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertablePower {
            creator_id,
            creature_id,
            name: "Generic Power".to_owned(),
            action_type: ActionType::Standard,
            target: ActionTarget::PhysicalDefense,
            resisted_by: ResistedBy::Physical,
            action_step: 9,
            effect_step: Some(9),
            details: Some("A basic power".to_owned()),
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

        InsertablePower {
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

