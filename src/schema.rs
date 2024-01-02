// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "action_targets"))]
    pub struct ActionTargets;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "action_types"))]
    pub struct ActionTypes;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "locales"))]
    pub struct Locales;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "rarities"))]
    pub struct Rarities;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "resisted_bys"))]
    pub struct ResistedBys;
}

diesel::table! {
    attacks (id) {
        id -> Uuid,
        creator_id -> Uuid,
        creature_id -> Uuid,
        name -> Varchar,
        action_step -> Int4,
        effect_step -> Int4,
        details -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        en_string -> Varchar,
        fr_string -> Varchar,
        en_description -> Nullable<Text>,
        fr_description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Locales;
    use super::sql_types::Rarities;

    creatures (id) {
        id -> Uuid,
        creator_id -> Uuid,
        name -> Varchar,
        found_in -> Array<Nullable<Locales>>,
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
        movement -> Varchar,
        recovery_rolls -> Int4,
        karma -> Int4,
        slug -> Varchar,
        image_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    documents (id) {
        id -> Uuid,
        template_id -> Uuid,
        title_text_id -> Uuid,
        purpose_text_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        security_classification -> Varchar,
        published -> Bool,
        created_by_id -> Uuid,
    }
}

diesel::table! {
    email_verification_code (id) {
        id -> Uuid,
        email_address -> Varchar,
        activation_code -> Varchar,
        expires_on -> Timestamp,
    }
}

diesel::table! {
    keywords (id) {
        id -> Uuid,
        en_string -> Varchar,
        fr_string -> Varchar,
        en_description -> Nullable<Text>,
        fr_description -> Nullable<Text>,
    }
}

diesel::table! {
    metadata (id) {
        id -> Uuid,
        searchable_title_en -> Varchar,
        searchable_title_fr -> Varchar,
        document_id -> Uuid,
        author_id -> Uuid,
        subject_id -> Nullable<Uuid>,
        category_id -> Nullable<Uuid>,
        summary_text_en -> Text,
        summary_text_fr -> Text,
        keyword_ids -> Nullable<Array<Nullable<Uuid>>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    password_reset_token (id) {
        id -> Uuid,
        email_address -> Varchar,
        reset_token -> Varchar,
        expires_on -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActionTypes;
    use super::sql_types::ActionTargets;
    use super::sql_types::ResistedBys;

    powers (id) {
        id -> Uuid,
        creator_id -> Uuid,
        creature_id -> Uuid,
        name -> Varchar,
        action_type -> ActionTypes,
        target -> ActionTargets,
        resisted_by -> ResistedBys,
        action_step -> Int4,
        effect_step -> Int4,
        details -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sections (id) {
        id -> Uuid,
        document_id -> Uuid,
        template_section_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by_id -> Uuid,
    }
}

diesel::table! {
    subjects (id) {
        id -> Uuid,
        en_string -> Varchar,
        fr_string -> Varchar,
        en_description -> Nullable<Text>,
        fr_description -> Nullable<Text>,
    }
}

diesel::table! {
    template_sections (id) {
        id -> Uuid,
        template_id -> Uuid,
        header_text_id -> Uuid,
        order_number -> Int4,
        help_text_id -> Uuid,
        character_limit -> Nullable<Int4>,
    }
}

diesel::table! {
    templates (id) {
        id -> Uuid,
        name_text_id -> Uuid,
        purpose_text_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        slug -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    texts (id, lang) {
        id -> Uuid,
        section_id -> Nullable<Uuid>,
        lang -> Varchar,
        content -> Array<Nullable<Text>>,
        keywords -> Nullable<Jsonb>,
        translated -> Array<Nullable<Bool>>,
        machine_translation -> Array<Nullable<Bool>>,
        created_at -> Array<Nullable<Timestamp>>,
        created_by_id -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        user_name -> Varchar,
        slug -> Varchar,
        created_at -> Timestamp,
        role -> Varchar,
        validated -> Bool,
    }
}

diesel::joinable!(attacks -> creatures (creature_id));
diesel::joinable!(attacks -> users (creator_id));
diesel::joinable!(creatures -> users (creator_id));
diesel::joinable!(documents -> templates (template_id));
diesel::joinable!(metadata -> documents (document_id));
diesel::joinable!(powers -> creatures (creature_id));
diesel::joinable!(powers -> users (creator_id));
diesel::joinable!(sections -> documents (document_id));
diesel::joinable!(sections -> template_sections (template_section_id));
diesel::joinable!(template_sections -> templates (template_id));
diesel::joinable!(texts -> sections (section_id));

diesel::allow_tables_to_appear_in_same_query!(
    attacks,
    categories,
    creatures,
    documents,
    email_verification_code,
    keywords,
    metadata,
    password_reset_token,
    powers,
    sections,
    subjects,
    template_sections,
    templates,
    texts,
    users,
);
