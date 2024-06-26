use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::database::connection;
use crate::schema::creatures;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};
use diesel_derive_enum::DbEnum;

use inflector::Inflector;
use strum::EnumString;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = creatures)]
pub struct Creature {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creator_slug: String,
    pub name: String,
    pub found_in: Vec<Option<Locales>>,
    pub rarity: Rarity,
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
    pub slug: String,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tags: Vec<Option<Tags>>,
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
            .filter(creatures::name.eq(&creature.name))
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

        let res: Creature = creatures::table
            .filter(creatures::id.eq(id))
            .first(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_name(name: &String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::name.ilike(format!("%{}%", name)))
            .load::<Creature>(&mut conn)?;

        Ok(res)
    }

    pub fn get_by_slug(slug: &String) -> Result<Self, CustomError> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::slug.eq(slug))
            .first::<Creature>(&mut conn)?;

        Ok(res)
    }

    // Searches text string in creature name and description
    pub fn search_by(text: String) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res = creatures::table
            .filter(creatures::name.ilike(format!("%{}%",text)))
            .or_filter(creatures::description.ilike(format!("%{}%", text)))
            .load::<Creature>(&mut conn)?;

        Ok(res)
    }

    pub fn search_by_location(location: Locales) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let location = vec![Some(location)];

        let res = creatures::table
            .filter(creatures::found_in.overlaps_with(location))
            .load::<Creature>(&mut conn)?;

        Ok(res)
    }

    pub fn search_by_tag(tag: Tags) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let tag = vec![Some(tag)];

        let res = creatures::table
            .filter(creatures::tags.overlaps_with(tag))
            .load::<Creature>(&mut conn)?;

        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;

        let res: Vec<Self> = creatures::table
            .load::<Creature>(&mut conn)?;

        Ok(res)
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

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;

        let res = diesel::delete(creatures::table.filter(
                creatures::id.eq(id)))
            .execute(&mut conn)?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, QueryId, EnumString)]
#[ExistingTypePath = "crate::schema::sql_types::Locales"]
pub enum Locales {
    #[strum(serialize = "Jungle", serialize = "jungle")]
    Jungle,
    #[strum(serialize = "Desert", serialize = "desert")]
    Desert,
    #[strum(serialize = "Forest", serialize = "forest")]
    Forest,
    #[strum(serialize = "Plains", serialize = "plains")]
    Plains,
    #[strum(serialize = "Urban", serialize = "urban")]
    Urban,
    #[strum(serialize = "Mountain", serialize = "mountain")]
    Mountain,
    #[strum(serialize = "Cavern", serialize = "cavern")]
    Cavern,
    #[strum(serialize = "Swamp", serialize = "swamp")]
    Swamp,
    #[strum(serialize = "Kaer", serialize = "kaer")]
    Kaer,
    #[strum(serialize = "Any", serialize = "any")]
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize, QueryId, EnumString)]
#[ExistingTypePath = "crate::schema::sql_types::Tags"]
pub enum Tags {
    #[strum(serialize = "Creature", serialize = "creature")]
    Creature,
    #[strum(serialize = "Spirit", serialize = "spirit")]
    Spirit,
    #[strum(serialize = "Elemental", serialize = "elemental")]
    Elemental,
    #[strum(serialize = "Horror", serialize = "horror")]
    Horror,
    #[strum(serialize = "Dragon", serialize = "dragon")]
    Dragon,
    #[strum(serialize = "HorrorConstruct", serialize = "horror_construct")]
    HorrorConstruct,
    #[strum(serialize = "Adept", serialize = "adept")]
    Adept,
    #[strum(serialize = "NPC", serialize = "npc")]
    NPC,
    #[strum(serialize = "Other", serialize = "other")]
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = creatures)]
pub struct InsertableCreature {
    pub creator_id: Uuid,
    pub creator_slug: String,
    pub name: String,
    pub found_in: Vec<Option<Locales>>,
    pub rarity: Rarity,
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
    pub slug: String,
    pub image_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub tags: Vec<Option<Tags>>,
}

impl InsertableCreature {

    pub fn default(creator_id: Uuid, creator_slug: String) -> Self {

        let locales = vec![Some(Locales::Jungle)];
        let today = chrono::Utc::now().naive_utc();

        InsertableCreature {
            creator_id,
            creator_slug,
            name: "".to_string(),
            found_in: locales,
            rarity: Rarity::Rare,
            circle_rank: 5,
            description: "Some description".to_owned(),
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
            movement: "10".to_string(),
            recovery_rolls: 3,
            karma: 0,
            slug: "esparaga".to_owned(),
            image_url: Some("hdahdksfashf".to_string()),
            created_at: today,
            updated_at: today,
            tags: vec![Some(Tags::Creature)],
        }
    }

    pub fn new(
        creator_id: Uuid,
        creator_slug: String,
        name: String,
        found_in: Vec<Option<Locales>>,
        rarity: Rarity,
        circle_rank: i32,
        description: String,
        dexterity: i32,
        strength: i32,
        constitution: i32,
        perception: i32,
        willpower: i32,
        charisma: i32,
        initiative: i32,
        pd: i32,
        md: i32,
        sd: i32,
        pa: i32,
        ma: i32,
        unconsciousness_rating: i32,
        death_rating: i32,
        wound: i32,
        knockdown: i32,
        actions: i32,
        movement: String,
        recovery_rolls: i32,
        karma: i32,
        tags: Vec<Option<Tags>>,
    ) -> Self {

        let slug = name.trim().to_snake_case();
        let today = chrono::Utc::now().naive_utc();

        InsertableCreature {
            creator_id,
            creator_slug,
            name,
            found_in,
            rarity,
            circle_rank,
            description,
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
            movement,
            recovery_rolls,
            karma,
            slug,
            image_url: Some("default_image_url".to_string()),
            created_at: today,
            updated_at: today,
            tags,
        }
    }
}

