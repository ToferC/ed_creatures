use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::schema::*;
use crate::database::connection;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};

use crate::models::Creature;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Associations, Identifiable, Clone)]
#[diesel(table_name = maneuvers)]
#[belongs_to(Creature)]
pub struct Maneuver {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub source: String,
    pub details: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Maneuver {
    pub fn create(maneuver_data: &InsertableManeuver) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(maneuvers::table)
            .values(maneuver_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(maneuver: &InsertableManeuver) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = maneuvers::table
            .filter(maneuvers::name.eq(&maneuver.name))
            .distinct()
            .first(&mut conn);

        let maneuver = match res {
            Ok(c) => c,
            Err(e) => {
                // maneuver not found
                println!("{:?}", e);
                let c = Maneuver::create(maneuver).expect("Unable to create maneuver");
                c
            }
        };
        Ok(maneuver)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = maneuvers::table
            .filter(maneuvers::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = maneuvers::table
            .filter(maneuvers::name.ilike(format!("%{}%", name)))
            .load::<Maneuver>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_creature_id(creature_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res: Vec<Self> = maneuvers::table
            .filter(maneuvers::creature_id.eq(creature_id))
            .load::<Maneuver>(&mut conn)?;

        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(maneuvers::table)
            .filter(maneuvers::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;

        let res = diesel::delete(maneuvers::table.filter(
                maneuvers::id.eq(id)))
            .execute(&mut conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = maneuvers)]
pub struct InsertableManeuver {
    pub creator_id: Uuid,
    pub creature_id: Uuid,
    pub name: String,
    pub source: String,
    pub details: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableManeuver {

    pub fn default(creator_id: Uuid, creature_id: Uuid) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableManeuver {
            creator_id,
            creature_id,
            name: "Melee Maneuver".to_owned(),
            source: "Creature".to_owned(),
            details: "A basic maneuver".to_owned(),
            created_at: today,
            updated_at: today,
        }
    }

    pub fn new(
        creator_id: Uuid,
        creature_id: Uuid,
        name: String,
        source: String,
        details: String,
    ) -> Self {

        let today = chrono::Utc::now().naive_utc();

        InsertableManeuver {
            creator_id,
            creature_id,
            name,
            source,
            details,
            created_at: today,
            updated_at: today,
        }
    }
}

