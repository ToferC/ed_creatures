use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::schema::creatures;
use crate::database::connection;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};
use diesel_derive_enum::DbEnum;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, AsChangeset, Clone)]
#[table_name = "creatures"]
pub struct Creature {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creature_name: String,
    pub found_in: Vec<Locales>,
    pub rarity: Rarity,
    pub circle_rank: u32,
    pub dexterity: u32,
    pub strength: u32,
    pub constitution: u32,
    pub perception: u32,
    pub willpower: u32,
    pub charisma: u32,
    pub initiative: u32,
    pub pd: u32,
    pub md: u32,
    pub sd: u32,
    pub pa: u32,
    pub ma: u32,
    pub unconsciousness_rating: u32,
    pub death_rating: u32,
    pub wound: u32,
    pub knockdown: u32,
    pub actions: u32,
    pub recovery_rolls: u32,
    pub slug: String,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Creature {
    pub fn create(creature_data: &InsertableCreature) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(creatures::table)
            .values(creature_data)
            .get_result(&mut conn)?;

        Ok(res)
    }

    pub fn get_or_create(creature: &InsertableCreature) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = creatures::table
            .filter(creatures::creature_name.eq(&creature.creature_name))
            .distinct()
            .first(&mut conn);

        let creature = match res {
            Ok(c) => c,
            Err(e) => {
                // creature not found
                println!("{:?}", e);
                let c = Creature::create(creature).expect("Unable to create creature");
                c
            }
        };
        Ok(creature)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::creature_name.ilike(format!("%{}%", name)))
            .load::<Creature>(&mut conn)?;
    }

    pub fn get_by_slug(slug: &String) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::slug.eq(slug))
            .first::<Creature>(&mut conn)?;
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        self.updated_at = chrono::Utc::now().naive_utc();

        let res = diesel::update(creatures::table)
            .filter(creatures::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Rarities"]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::Locales"]
pub enum Locales {
    Jungle,
    Desert,
    Forest,
    Plains,
    Urban,
    Mountain,
    Cavern,
    Kaer,
    Any,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
/// Referenced by Roles, TeamOwnership, OrgOwnership
#[diesel(table_name = creatures)]
pub struct InsertableCreature {
    pub creator_id: Uuid,
    pub creature_name: String,
    pub found_in: Vec<Locales>,
    pub rarity: Rarity,
    pub circle_rank: u32,
    pub dexterity: u32,
    pub strength: u32,
    pub constitution: u32,
    pub perception: u32,
    pub willpower: u32,
    pub charisma: u32,
    pub initiative: u32,
    pub pd: u32,
    pub md: u32,
    pub sd: u32,
    pub pa: u32,
    pub ma: u32,
    pub unconsciousness_rating: u32,
    pub death_rating: u32,
    pub wound: u32,
    pub knockdown: u32,
    pub actions: u32,
    pub recovery_rolls: u32,
    pub slug: String,
    pub image_url: String,
}

impl InsertableCreature {

    pub fn default(creator_id: Uuid) -> Self {

        let locales = Locales::Jungle;
        let today = chrono::Utc::now().naive_utc();

        InsertableCreature {
            creator_id,
            creature_name: "Esparaga".to_string(),
            found_in: locales,
            rarity: Rarity::Rare,
            circle_rank: 5,
            dexterity: 10,
            strength: 10,
            constitution: 10,
            perception: 10,
            willpower: 10,
            charisma: 10,
            initiative: 10,
            pd: 9,
            md: 9,
            sd: 9,
            pa: 9,
            ma: 9,
            unconsciousness_rating: 45,
            death_rating: 55,
            wound: 12,
            knockdown: 10,
            actions: 2,
            recovery_rolls: 3,
            image_url: "hdahdksfashf".to_string(),
            slug: "esparaga".to_owned(),
        }
    }

    pub fn new(
        creator_id: Uuid,
        creature_name: String,
        found_in: Vec<Locales>,
        rarity: Rarity,
        circle_rank: u32,
        dexterity: u32,
        strength: u32,
        constitution: u32,
        perception: u32,
        willpower: u32,
        charisma: u32,
        initiative: u32,
        pd: u32,
        md: u32,
        sd: u32,
        pa: u32,
        ma: u32,
        unconsciousness_rating: u32,
        death_rating: u32,
        wound: u32,
        knockdown: u32,
        actions: u32,
        recovery_rolls: u32,
    ) -> Self {

        let slug = creature_name.trim().to_snake_case();

        InsertableCreature {
            creator_id,
            creature_name,
            found_in,
            rarity,
            circle_rank,
            dexterity,
            strength,
            constitution,
            perception,
            willpower,
            charisma,
            initiative,
            pd,
            md,
            sd,
            pa,
            ma,
            unconsciousness_rating,
            death_rating,
            wound,
            knockdown,
            actions,
            recovery_rolls,
            slug,
            image_url: "default_image_url".as_string(),
        }
    }
}

