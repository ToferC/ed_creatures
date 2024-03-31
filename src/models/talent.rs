use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::schema::talents;
use crate::database::connection;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};

use crate::models::Creature;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Associations, Identifiable, Clone)]
#[diesel(table_name = talents)]
#[belongs_to(Creature)]
pub struct Talent {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Talent {
    pub fn create(talent_data: &InsertableTalent) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(talents::table)
            .values(talent_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(talent: &InsertableTalent) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = talents::table
            .filter(talents::name.eq(&talent.name))
            .distinct()
            .first(&mut conn);

        let talent = match res {
            Ok(c) => c,
            Err(e) => {
                // talent not found
                println!("{:?}", e);
                let c = Talent::create(talent).expect("Unable to create talent");
                c
            }
        };
        Ok(talent)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = talents::table
            .filter(talents::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = talents::table
            .filter(talents::name.ilike(format!("%{}%", name)))
            .load::<Talent>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_creature_id(creature_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res: Vec<Self> = talents::table
            .filter(talents::creature_id.eq(creature_id))
            .load::<Talent>(&mut conn)?;

        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(talents::table)
            .filter(talents::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;

        let res = diesel::delete(talents::table.filter(
                talents::id.eq(id)))
            .execute(&mut conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = talents)]
pub struct InsertableTalent {
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableTalent {

    pub fn default(creator_id: Uuid, creature_id: Uuid) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableTalent {
            creator_id,
            creature_id,
            name: "Melee Talent".to_owned(),
            action_step: 9,
            effect_step: 9,
            details: Some("A basic talent".to_owned()),
            created_at: today,
            updated_at: today,
        }
    }

    pub fn new(
        creator_id: Uuid,
        creature_id: Uuid,
        name: String,
        action_step: i32,
    ) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableTalent {
            creator_id,
            creature_id,
            name,
            action_step,
            created_at: today,
            updated_at: today,
        }
    }
}

