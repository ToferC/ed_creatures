use actix_web::{HttpServer, App, middleware, web};
use dotenv::dotenv;
use std::env;
use tera::Tera;
use tera_text_filters::snake_case;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web_static_files;
use sendgrid::SGClient;

use web_starter::APP_NAME;
use web_starter::handlers;
use web_starter::AppData;
use web_starter::database;

use fluent_templates::{FluentLoader, static_loader};
// https://lib.rs/crates/fluent-templates

// Setup for serving static files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static_loader! {
    static LOCALES = {
        locales: "./i18n/",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let environment = env::var("ENVIRONMENT");

    let environment = match environment {
        Ok(v) => v,
        Err(_) => String::from("test"),
    };

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8080"))
    };

    let cookie_secret = env::var("COOKIE_SECRET_KEY").expect("Unable to find secret key");

    let cookie_secret_key: Key = Key::from(&cookie_secret.as_bytes());
    
    database::init();

    // SendGrid email API
    let sendgrid_var = env::var("SENDGRID_API_KEY");
    let sendgrid_key: String;

    match sendgrid_var {
        Ok(key) => sendgrid_key = key,
        Err(err) => panic!("Must supply API key in env variables to use: {}", err),
    };

    println!("Serving {} on {}:{}", APP_NAME, &host, &port);

    HttpServer::new(move || {
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto-reload with Tera");
        tera.register_function("fluent", FluentLoader::new(&*LOCALES));

        // mail client
        let sg = SGClient::new(sendgrid_key.clone());

        let data = web::Data::new(AppData {
            tmpl: tera,
            mail_client: sg,
        });

        let generated = generate();

        App::new()
            .wrap(middleware::Logger::default())
            .configure(handlers::configure_services)
            .app_data(data.clone())
            .service(actix_web_static_files::ResourceFiles::new(
                "/static", generated,
            ))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(), cookie_secret_key.clone())
                    .cookie_secure(false)
                    .build()
                )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}