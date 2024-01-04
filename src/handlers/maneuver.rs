use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest, put};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData, models::{User, Maneuver, InsertableManeuver}, handlers::ManeuverForm};
use uuid::Uuid;

#[get("/{lang}/maneuvers/{creature_id}")]
pub async fn get_maneuvers(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let maneuvers = Maneuver::get_by_creature_id(creature_id).expect("Unable to retrieve maneuver");

    ctx.insert("maneuvers", &maneuvers);
    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("maneuvers/maneuvers.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/maneuver/{maneuver_id}")]
pub async fn get_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, maneuver_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let maneuver = Maneuver::get_by_id(&maneuver_id).expect("Unable to retrieve maneuver");

    ctx.insert("maneuver", &maneuver);
    ctx.insert("creature_id", &maneuver.creature_id);

    let rendered = data.tmpl.render("maneuvers/maneuver.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/add_maneuver/{creature_id}")]
pub async fn add_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("maneuvers/add_maneuver.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_maneuver/{creature_id}")]
pub async fn post_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<ManeuverForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());
    
    let user = User::get_from_slug(&session_user);

    if let Err(_e) = user {
        // no user found so redirect to home
        return HttpResponse::Found()
        .append_header(("Location", format!("/{}/", &lang))).finish()
    }

    let user = user.unwrap();

    let new_maneuver = InsertableManeuver::new(
        user.id,
        creature_id,
        form.name.to_owned(),
        form.source.to_owned(),
        form.details.to_owned(),
        );

    let _maneuver = Maneuver::create(&new_maneuver).expect("Unable to create maneuver");

    println!("Saved!");

    let result = Maneuver::get_by_creature_id(creature_id);

    let maneuvers = match result {
        Ok(a) => a,
        Err(e) => {
            // Unable to retrieve maneuvers
            // validate form has data or and permissions exist
            println!("{:?}", e);
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &creature_id))).finish()
        }
    };


    ctx.insert("maneuvers", &maneuvers);
    ctx.insert("creature_id", &creature_id);

    //Redirect to get maneuver with maneuver slug
    let rendered = data.tmpl.render("maneuvers/maneuvers.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_maneuver/{maneuver_id}")]
pub async fn edit_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, maneuver_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let maneuver = Maneuver::get_by_id(&maneuver_id).expect("Unable to retrieve maneuver");

    ctx.insert("maneuver", &maneuver);

    let rendered = data.tmpl.render("maneuvers/edit_maneuver_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_maneuver_post/{maneuver_id}")]
pub async fn edit_maneuver_post(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<ManeuverForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, maneuver_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user).expect("Unable to retrive user");

    let result = Maneuver::get_by_id(&maneuver_id);

    let maneuver = match result {
        Ok(c) => c,
        Err(r) => {
            // Unable to retrieve maneuver
            // validate form has data or and permissions exist
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_maneuver/{}", &lang, &maneuver_id))).finish()
        }
    };

    let today = chrono::Utc::now().naive_utc();

    let mut our_maneuver = Maneuver {
        id: maneuver.id,
        creator_id: user.id,
        creature_id: maneuver.creature_id,
        name: form.name.to_owned(),
        source: form.source.to_owned(),
        details: form.details.to_owned(),
        created_at: maneuver.created_at,
        updated_at: today,
    };

    let maneuver = Maneuver::update(&mut our_maneuver).expect("Unable to create maneuver");

    println!("Saved!");

    ctx.insert("maneuver", &maneuver);

    // Redirect to maneuver
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/maneuver/{}", &lang, &maneuver.id))).finish()
}

#[get("/{lang}/delete_maneuver/{maneuver_id}")]
pub async fn delete_maneuver(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, maneuver_id) = path.into_inner();

    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let _maneuver = Maneuver::delete(maneuver_id).expect("Unable to delete maneuver");

    HttpResponse::Ok().body("")
}