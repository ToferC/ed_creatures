use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::schema::attacks;
use crate::database::connection;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = attacks)]
pub struct Attack {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub effect_step: i32,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Attack {
    pub fn create(attack_data: &InsertableAttack) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(attacks::table)
            .values(attack_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(attack: &InsertableAttack) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = attacks::table
            .filter(attacks::name.eq(&attack.name))
            .distinct()
            .first(&mut conn);

        let attack = match res {
            Ok(c) => c,
            Err(e) => {
                // attack not found
                println!("{:?}", e);
                let c = Attack::create(attack).expect("Unable to create attack");
                c
            }
        };
        Ok(attack)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = attacks::table
            .filter(attacks::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = attacks::table
            .filter(attacks::name.ilike(format!("%{}%", name)))
            .load::<Attack>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_creature_id(creature_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res: Vec<Self> = attacks::table
            .filter(attacks::creature_id.eq(creature_id))
            .load::<Attack>(&mut conn)?;

        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(attacks::table)
            .filter(attacks::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;

        let res = diesel::delete(attacks::table.filter(
                attacks::id.eq(id)))
            .execute(&mut conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = attacks)]
pub struct InsertableAttack {
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub effect_step: i32,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableAttack {

    pub fn default(creator_id: Uuid, creature_id: Uuid) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableAttack {
            creator_id,
            creature_id,
            name: "Melee Attack".to_owned(),
            action_step: 9,
            effect_step: 9,
            details: Some("A basic attack".to_owned()),
            created_at: today,
            updated_at: today,
        }
    }

    pub fn new(
        creator_id: Uuid,
        creature_id: Uuid,
        name: String,
        action_step: i32,
        effect_step: i32,
        details: Option<String>,
    ) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableAttack {
            creator_id,
            creature_id,
            name,
            action_step,
            effect_step,
            details,
            created_at: today,
            updated_at: today,
        }
    }
}

