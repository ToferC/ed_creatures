use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData, models::{User, Power}, handlers::PowerForm};
use uuid::Uuid;

use crate::models::{InsertablePower};

#[get("/{lang}/powers/{creature_id}")]
pub async fn get_powers(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let powers = Power::get_by_creature_id(creature_id).expect("Unable to retrieve power");

    ctx.insert("powers", &powers);
    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("powers/powers.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/power/{power_id}")]
pub async fn get_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, power_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let power = Power::get_by_id(&power_id).expect("Unable to retrieve power");

    ctx.insert("power", &power);
    ctx.insert("creature_id", &power.creature_id);

    let rendered = data.tmpl.render("powers/power.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/add_power/{creature_id}")]
pub async fn add_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("powers/add_power.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_power/{creature_id}")]
pub async fn post_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<PowerForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());
    
    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        // no user found so redirect to home
        println!("Error: {:?}", &e);
        return HttpResponse::Found()
        .append_header(("Location", format!("/{}/", &lang))).finish()
    }

    let user = user.unwrap();

    let new_power = InsertablePower::new(
        user.id,
        creature_id,
        form.name.to_owned(),
        form.action_type,
        form.target,
        form.resisted_by,
        form.action_step,
        form.effect_step,
        form.details.to_owned(),
        );

    let _power = Power::create(&new_power).expect("Unable to create power");

    println!("Saved!");

    let result = Power::get_by_creature_id(creature_id);

    let powers = match result {
        Ok(a) => a,
        Err(e) => {
            // Unable to retrieve powers
            // validate form has data or and permissions exist
            println!("{:?}", e);
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &creature_id))).finish()
        }
    };


    ctx.insert("powers", &powers);
    ctx.insert("creature_id", &creature_id);

    //Redirect to get power with power slug
    let rendered = data.tmpl.render("powers/powers.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_power/{power_id}")]
pub async fn edit_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, power_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let power = Power::get_by_id(&power_id).expect("Unable to retrieve power");

    ctx.insert("power", &power);

    let rendered = data.tmpl.render("powers/edit_power_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_power_post/{power_id}")]
pub async fn edit_power_post(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<PowerForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, power_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user).expect("Unable to retrive user");

    let result = Power::get_by_id(&power_id);

    let power = match result {
        Ok(c) => c,
        Err(e) => {
            // Unable to retrieve power
            println!("Error: {:?}", &e);
            // validate form has data or and permissions exist
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_power/{}", &lang, &power_id))).finish()
        }
    };

    let today = chrono::Utc::now().naive_utc();

    let mut our_power = Power {
        id: power.id,
        creator_id: user.id,
        creature_id: power.creature_id,
        name: form.name.to_owned(),
        action_type: form.action_type,
        target: form.target,
        resisted_by: form.resisted_by,
        action_step: form.action_step,
        effect_step: form.effect_step,
        details: form.details.to_owned(),
        created_at: power.created_at,
        updated_at: today,
    };

    let power = Power::update(&mut our_power).expect("Unable to create power");

    println!("Saved!");

    ctx.insert("power", &power);

    // Redirect to power
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/power/{}", &lang, &power.id))).finish()
}

#[get("/{lang}/delete_power/{power_id}")]
pub async fn delete_power(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, power_id) = path.into_inner();

    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let _attack = Power::delete(power_id).expect("Unable to delete attack");

    HttpResponse::Ok().body("")
}