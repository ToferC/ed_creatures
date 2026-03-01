use chrono::NaiveDateTime;
use uuid::Uuid;

use serde::{Serialize, Deserialize};

use crate::errors::CustomError;
use crate::database::connection;
use crate::schema::{masks, mask_attacks, mask_powers, mask_talents, mask_maneuvers};
use crate::models::{ActionTarget, ActionType, ResistedBy};

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl};

// ---- Mask ----

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = masks)]
pub struct Mask {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub creator_slug: String,
    pub name: String,
    pub description: String,
    // stat deltas - added to creature's current stats when applied
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
    pub recovery_rolls: i32,
    pub karma: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug, Clone)]
pub struct MaskRef {
    pub id: Uuid,
    pub name: String,
}

impl Mask {
    pub fn create(data: &InsertableMask) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(masks::table)
            .values(data)
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = masks::table
            .filter(masks::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;
        let res = masks::table
            .order(masks::name.asc())
            .load::<Mask>(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_creator(creator_id: &Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;
        let res = masks::table
            .filter(masks::creator_id.eq(creator_id))
            .order(masks::name.asc())
            .load::<Mask>(&mut conn)?;
        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        self.updated_at = chrono::Utc::now().naive_utc();
        let res = diesel::update(masks::table)
            .filter(masks::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;
        let res = diesel::delete(masks::table.filter(masks::id.eq(id)))
            .execute(&mut conn)?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = masks)]
pub struct InsertableMask {
    pub creator_id: Uuid,
    pub creator_slug: String,
    pub name: String,
    pub description: String,
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
    pub recovery_rolls: i32,
    pub karma: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableMask {
    pub fn new(
        creator_id: Uuid,
        creator_slug: String,
        name: String,
        description: String,
        circle_rank: i32,
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
        recovery_rolls: i32,
        karma: i32,
    ) -> Self {
        let today = chrono::Utc::now().naive_utc();
        InsertableMask {
            creator_id,
            creator_slug,
            name,
            description,
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
            karma,
            created_at: today,
            updated_at: today,
        }
    }
}

// ---- MaskAttack ----

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = mask_attacks)]
pub struct MaskAttack {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub mask_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub effect_step: i32,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MaskAttack {
    pub fn create(data: &InsertableMaskAttack) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(mask_attacks::table)
            .values(data)
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = mask_attacks::table
            .filter(mask_attacks::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_mask_id(mask_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;
        let res = mask_attacks::table
            .filter(mask_attacks::mask_id.eq(mask_id))
            .load::<MaskAttack>(&mut conn)?;
        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        self.updated_at = chrono::Utc::now().naive_utc();
        let res = diesel::update(mask_attacks::table)
            .filter(mask_attacks::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;
        let res = diesel::delete(mask_attacks::table.filter(mask_attacks::id.eq(id)))
            .execute(&mut conn)?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = mask_attacks)]
pub struct InsertableMaskAttack {
    pub creator_id: Uuid,
    pub mask_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub effect_step: i32,
    pub details: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableMaskAttack {
    pub fn new(
        creator_id: Uuid,
        mask_id: Uuid,
        name: String,
        action_step: i32,
        effect_step: i32,
        details: Option<String>,
    ) -> Self {
        let today = chrono::Utc::now().naive_utc();
        InsertableMaskAttack {
            creator_id,
            mask_id,
            name,
            action_step,
            effect_step,
            details,
            created_at: today,
            updated_at: today,
        }
    }
}

// ---- MaskPower ----

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = mask_powers)]
pub struct MaskPower {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub mask_id: Uuid,
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

impl MaskPower {
    pub fn create(data: &InsertableMaskPower) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(mask_powers::table)
            .values(data)
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = mask_powers::table
            .filter(mask_powers::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_mask_id(mask_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;
        let res = mask_powers::table
            .filter(mask_powers::mask_id.eq(mask_id))
            .load::<MaskPower>(&mut conn)?;
        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        self.updated_at = chrono::Utc::now().naive_utc();
        let res = diesel::update(mask_powers::table)
            .filter(mask_powers::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;
        let res = diesel::delete(mask_powers::table.filter(mask_powers::id.eq(id)))
            .execute(&mut conn)?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = mask_powers)]
pub struct InsertableMaskPower {
    pub creator_id: Uuid,
    pub mask_id: Uuid,
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

impl InsertableMaskPower {
    pub fn new(
        creator_id: Uuid,
        mask_id: Uuid,
        name: String,
        action_type: ActionType,
        target: ActionTarget,
        resisted_by: ResistedBy,
        action_step: i32,
        effect_step: Option<i32>,
        details: Option<String>,
    ) -> Self {
        let today = chrono::Utc::now().naive_utc();
        InsertableMaskPower {
            creator_id,
            mask_id,
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

// ---- MaskTalent ----

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = mask_talents)]
pub struct MaskTalent {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub mask_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MaskTalent {
    pub fn create(data: &InsertableMaskTalent) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(mask_talents::table)
            .values(data)
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = mask_talents::table
            .filter(mask_talents::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_mask_id(mask_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;
        let res = mask_talents::table
            .filter(mask_talents::mask_id.eq(mask_id))
            .load::<MaskTalent>(&mut conn)?;
        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        self.updated_at = chrono::Utc::now().naive_utc();
        let res = diesel::update(mask_talents::table)
            .filter(mask_talents::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;
        let res = diesel::delete(mask_talents::table.filter(mask_talents::id.eq(id)))
            .execute(&mut conn)?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = mask_talents)]
pub struct InsertableMaskTalent {
    pub creator_id: Uuid,
    pub mask_id: Uuid,
    pub name: String,
    pub action_step: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableMaskTalent {
    pub fn new(
        creator_id: Uuid,
        mask_id: Uuid,
        name: String,
        action_step: i32,
    ) -> Self {
        let today = chrono::Utc::now().naive_utc();
        InsertableMaskTalent {
            creator_id,
            mask_id,
            name,
            action_step,
            created_at: today,
            updated_at: today,
        }
    }
}

// ---- MaskManeuver ----

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = mask_maneuvers)]
pub struct MaskManeuver {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub mask_id: Uuid,
    pub name: String,
    pub source: String,
    pub details: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl MaskManeuver {
    pub fn create(data: &InsertableMaskManeuver) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = diesel::insert_into(mask_maneuvers::table)
            .values(data)
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_id(id: &Uuid) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        let res = mask_maneuvers::table
            .filter(mask_maneuvers::id.eq(id))
            .first(&mut conn)?;
        Ok(res)
    }

    pub fn get_by_mask_id(mask_id: Uuid) -> Result<Vec<Self>, CustomError> {
        let mut conn = connection()?;
        let res = mask_maneuvers::table
            .filter(mask_maneuvers::mask_id.eq(mask_id))
            .load::<MaskManeuver>(&mut conn)?;
        Ok(res)
    }

    pub fn update(&mut self) -> Result<Self, CustomError> {
        let mut conn = connection()?;
        self.updated_at = chrono::Utc::now().naive_utc();
        let res = diesel::update(mask_maneuvers::table)
            .filter(mask_maneuvers::id.eq(&self.id))
            .set(self.clone())
            .get_result(&mut conn)?;
        Ok(res)
    }

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let mut conn = connection()?;
        let res = diesel::delete(mask_maneuvers::table.filter(mask_maneuvers::id.eq(id)))
            .execute(&mut conn)?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = mask_maneuvers)]
pub struct InsertableMaskManeuver {
    pub creator_id: Uuid,
    pub mask_id: Uuid,
    pub name: String,
    pub source: String,
    pub details: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertableMaskManeuver {
    pub fn new(
        creator_id: Uuid,
        mask_id: Uuid,
        name: String,
        source: String,
        details: String,
    ) -> Self {
        let today = chrono::Utc::now().naive_utc();
        InsertableMaskManeuver {
            creator_id,
            mask_id,
            name,
            source,
            details,
            created_at: today,
            updated_at: today,
        }
    }
}
