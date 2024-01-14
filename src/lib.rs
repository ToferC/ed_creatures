pub mod models;
pub mod handlers;
pub mod database;
pub mod errors;
pub mod schema;
pub mod steps;

use models::UserRole;
use tera::{Tera, Context};
use actix_identity::Identity;
use actix_session::Session;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use sendgrid::SGClient;

#[macro_use]
extern crate diesel;

extern crate diesel_migrations;

pub const APP_NAME: &str = "Earthdawn Creatures";

#[derive(Clone, Debug)]
pub struct AppData {
    pub tmpl: Tera,
    pub mail_client: SGClient,
    pub steps: &'static [&'static str],
}

/// Generate context, session_user, role and node_names from id and lang
pub fn generate_basic_context(
    id: Option<Identity>,
    lang: &str,
    path: &str,
) -> (Context, String, UserRole, String) 
{    
    let mut ctx = Context::new();

    // Get session data and add to context
    let (session_user, role, _id) = extract_identity_data(id);
    ctx.insert("session_user", &session_user);
    ctx.insert("role", &role);

    let validated_lang = match lang {
        "fr" => "fr",
        "en" => "en",
        _ => "en",
    };

    ctx.insert("lang", &validated_lang);
    ctx.insert("path", &path);

    (ctx, session_user, role, lang.to_owned())
}

pub fn extract_session_data(session: &Session) -> (String, String) {

    let role_data = session.get::<String>("role").expect("Unable to get role from cookie");

    let role = match role_data {
        Some(r) => r,
        None => "".to_string(),
    };

    let user_data = session.get::<String>("user_name").expect("Unable to get user_name from cookie");

    let session_user = match user_data {
        Some(u) => u,
        None => "".to_string(),
    };

    println!("{}-{}", &session_user, &role);

    (session_user, role)
}

pub fn extract_identity_data(id: Option<Identity>) -> (String, UserRole, Option<Identity>) {

    if let Some(id) = id {
        
        let id_data = id.id();
    
        let session_user = match id_data {
            Ok(u) => u,
            Err(_e) => "".to_string(),
        };
    
        let user = models::User::find_slim_from_slug(&session_user);
    
        let role = match user {
            Ok(u) => u.role,
            _ => models::UserRole::Visitor,
        };
    
        println!("{}-{:?}", &session_user, &role);
    
        (session_user, role, Some(id))
    } else {
        ("".to_string(), UserRole::Visitor, None)
    }

}

/// Generate context, session_user and role from id and lang
pub fn generate_email_context(
    session_user: String,
    role: UserRole,
    lang: &str,
    path: &str,) -> (Context, String, UserRole, String) 
{    
let mut ctx = Context::new();

// Get session data and add to context
ctx.insert("session_user", &session_user);
ctx.insert("role", &role);

let validated_lang = match lang {
    "fr" => "fr",
    "en" => "en",
    _ => "en",
};

ctx.insert("lang", &validated_lang);
ctx.insert("path", &path);

(ctx, session_user, role, lang.to_owned())
}

pub fn generate_unique_code(mut characters: usize, dashes: bool) -> String {

    if characters > 64 {
        characters = 64;
    };

    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(characters)
        .map(char::from)
        .collect();

    if dashes {
        for i in 0..rand_string.len() + rand_string.len() / 4 {
            if i > 2 && i % 4 == 0 {
                rand_string.insert(i, '-');
            }
        }
    };

    rand_string
}