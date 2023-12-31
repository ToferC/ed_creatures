// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "locales"))]
    pub struct Locales;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "priority_type"))]
    pub struct PriorityType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rarities"))]
    pub struct Rarities;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Locales;
    use super::sql_types::Rarities;

    creatures (id) {
        id -> Uuid,
        creator_id -> Uuid,
        #[max_length = 128]
        creature_name -> Varchar,
        found_in -> Locales,
        rarity -> Rarities,
        circle_rank -> Int4,
        dexterity -> Int4,
        strength -> Int4,
        constitution -> Int4,
        perception -> Int4,
        willpower -> Int4,
        charisma -> Int4,
        initiative -> Int4,
        pd -> Int4,
        md -> Int4,
        sd -> Int4,
        pa -> Int4,
        ma -> Int4,
        unconsciousness_rating -> Int4,
        death_rating -> Int4,
        wound -> Int4,
        knockdown -> Int4,
        actions -> Int4,
        recovery_rolls -> Int4,
        #[max_length = 128]
        slug -> Varchar,
        #[max_length = 512]
        image_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    email_verification_code (id) {
        id -> Uuid,
        #[max_length = 128]
        email_address -> Varchar,
        #[max_length = 5]
        activation_code -> Varchar,
        expires_on -> Timestamp,
    }
}

diesel::table! {
    password_reset_token (id) {
        id -> Uuid,
        #[max_length = 128]
        email_address -> Varchar,
        #[max_length = 36]
        reset_token -> Varchar,
        expires_on -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PriorityType;

    todos (id) {
        id -> Uuid,
        list_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        priority -> PriorityType,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    todos_list (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        hash -> Bytea,
        #[max_length = 255]
        salt -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 32]
        user_name -> Varchar,
        #[max_length = 32]
        slug -> Varchar,
        created_at -> Timestamp,
        #[max_length = 32]
        role -> Varchar,
        validated -> Bool,
    }
}

diesel::joinable!(creatures -> users (creator_id));
diesel::joinable!(todos -> todos_list (list_id));
diesel::joinable!(todos_list -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    creatures,
    email_verification_code,
    password_reset_token,
    todos,
    todos_list,
    users,
);
