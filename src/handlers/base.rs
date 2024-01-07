use std::str::FromStr;

use actix_web::{web, get, Responder, HttpResponse, HttpRequest, post};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData};

use crate::models::{Creature, Locales};
use super::SearchForm;

#[get("/")]
pub async fn raw_index() -> impl Responder {
    return HttpResponse::Found().append_header(("Location", "/en")).finish()
}

#[get("/{lang}")]
pub async fn index(
    data: web::Data<AppData>,
    path: web::Path<String>,

    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());

    let creatures = Creature::get_all().expect("Unable to get lists");

    ctx.insert("creatures", &creatures);

    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)

}

#[post("/{lang}/search")]
pub async fn search(
    data: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Form<SearchForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let lang = path.into_inner();

    let (mut ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());

    let creatures = Creature::search_by(form.search.to_owned()).expect("Unable to get creatures from search");

    ctx.insert("creatures", &creatures);
    ctx.insert("search_text", &form.search);

    let rendered = data.tmpl.render("search_results.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/location/{location}")]
pub async fn creature_by_location(
    data: web::Data<AppData>,
    path: web::Path<(String, String)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, location) = path.into_inner();

    let (mut ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());

    let locale = Locales::from_str(&location).unwrap();

    let creatures = Creature::search_by_location(locale).expect("Unable to get creatures from search");

    ctx.insert("creatures", &creatures);
    ctx.insert("search_text", &location);

    let rendered = data.tmpl.render("search_results.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}