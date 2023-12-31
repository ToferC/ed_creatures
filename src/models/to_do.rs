use uuid::Uuid;
use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::schema::{todos, todos_list};
use diesel_derive_enum::DbEnum;

use crate::database;
use crate::errors::CustomError;

use shrinkwraprs::Shrinkwrap;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{self, Insertable, Queryable, ExpressionMethods};


#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = todos)]
pub struct ToDo {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: PriorityType,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = todos)]
pub struct InsertableToDo {
    pub list_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: PriorityType,
    pub active: bool,
}

impl InsertableToDo {
    pub fn new(
        list_id: Uuid,
        title: String,
        description: Option<String>,
        priority: PriorityType,
        active: bool,
    ) -> Self {

        InsertableToDo {
            list_id,
            title,
            description,
            priority,
            active,
        }
    }
}

impl ToDo {
    pub fn create(new_todo: &InsertableToDo) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;

        let result = diesel::insert_into(todos::table)
            .values(new_todo)
            .get_result(&mut conn)?;

        Ok(result)
    }

    pub fn batch_create(todos: Vec<InsertableToDo>) -> Result<usize, CustomError> {
        let mut conn = database::connection()?;

        let res = diesel::insert_into(todos::table)
            .values(todos)
            .execute(&mut conn)?;
        
        Ok(res)
    }

    pub fn get(id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;

        let result: ToDo = todos::table.filter(todos::id.eq(id))
            .first(&mut conn)?;

        Ok(result)
    }

    pub fn toggle_status(self) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;

        let new_status = match self.active {
            true => false,
            false => true,
        };

        let result = diesel::update(todos::table)
            .filter(todos::id.eq(self.id))
            .set((todos::active.eq(new_status)))
            .get_result(&mut conn)?;

        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Queryable, Debug, Identifiable, AsChangeset, Clone, Insertable)]
#[diesel(table_name = todos_list)]
pub struct ToDoList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = todos_list)]
pub struct InsertableToDoList {
    pub user_id: Uuid,
}

impl ToDoList {
    pub fn create(user_id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;

        let new_list = InsertableToDoList {
            user_id,
        };

        let list = diesel::insert_into(todos_list::table)
            .values(new_list)
            .get_result(&mut conn)?;

        Ok(list)
    }

    pub fn get(id: Uuid) -> Result<Self, CustomError> {
        let mut conn = database::connection()?;

        let result: ToDoList = todos_list::table.filter(todos_list::id.eq(id))
            .first(&mut conn)?;

        Ok(result)
    }

    pub fn get_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = database::connection()?;
        let lists = todos_list::table
            .load::<ToDoList>(&mut conn)?;
        Ok(lists)
    }

    pub fn get_active_todos(id: Uuid) -> Result<Vec<ToDo>, CustomError> {
        let mut conn = database::connection()?;

        let result: Vec<ToDo> = todos::table.filter(todos::list_id.eq(id))
            .filter(todos::active.eq(true))
            .load::<ToDo>(&mut conn)?;

        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::PriorityType"]
pub enum PriorityType {
    Low,
    Medium,
    High,
}