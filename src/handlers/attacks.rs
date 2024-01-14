use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData, models::{User, Attack, InsertableAttack}, handlers::AttackForm};
use uuid::Uuid;

#[get("/{lang}/attacks/{creature_id}")]
pub async fn get_attacks(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let attacks = Attack::get_by_creature_id(creature_id).expect("Unable to retrieve attack");

    ctx.insert("attacks", &attacks);
    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("attacks/attacks.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/attack/{attack_id}")]
pub async fn get_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, attack_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let attack = Attack::get_by_id(&attack_id).expect("Unable to retrieve attack");

    ctx.insert("attack", &attack);
    ctx.insert("creature_id", &attack.creature_id);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("attacks/attack.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/add_attack/{creature_id}")]
pub async fn add_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("attacks/add_attack.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_attack/{creature_id}")]
pub async fn post_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<AttackForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());
    
    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        // no user found so redirect to home
        println!("{}", e);
        return HttpResponse::Found()
        .append_header(("Location", format!("/{}/", &lang))).finish()
    }

    let user = user.unwrap();

    let new_attack = InsertableAttack::new(
        user.id,
        creature_id,
        form.name.to_owned(),
        form.action_step,
        form.effect_step,
        form.details.to_owned(),
        );

    let _attack = Attack::create(&new_attack).expect("Unable to create attack");

    println!("Saved!");

    let result = Attack::get_by_creature_id(creature_id);

    let attacks = match result {
        Ok(a) => a,
        Err(e) => {
            // Unable to retrieve attacks
            // validate form has data or and permissions exist
            println!("{:?}", e);
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &creature_id))).finish()
        }
    };


    ctx.insert("attacks", &attacks);
    ctx.insert("creature_id", &creature_id);

    //Redirect to get attack with attack slug
    let rendered = data.tmpl.render("attacks/attacks.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_attack/{attack_id}")]
pub async fn edit_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, attack_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let attack = Attack::get_by_id(&attack_id).expect("Unable to retrieve attack");

    ctx.insert("attack", &attack);

    let rendered = data.tmpl.render("attacks/edit_attack_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_attack_post/{attack_id}")]
pub async fn edit_attack_post(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<AttackForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, attack_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user).expect("Unable to retrive user");

    let result = Attack::get_by_id(&attack_id);

    let attack = match result {
        Ok(c) => c,
        Err(_e) => {
            // Unable to retrieve attack
            // validate form has data or and permissions exist
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_attack/{}", &lang, &attack_id))).finish()
        }
    };

    let today = chrono::Utc::now().naive_utc();

    let mut our_attack = Attack {
        id: attack.id,
        creator_id: user.id,
        creature_id: attack.creature_id,
        name: form.name.to_owned(),
        action_step: form.action_step,
        effect_step: form.effect_step,
        details: form.details.to_owned(),
        created_at: attack.created_at,
        updated_at: today,
    };

    let attack = Attack::update(&mut our_attack).expect("Unable to create attack");

    println!("Saved!");

    ctx.insert("attack", &attack);

    // Redirect to attack
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/attack/{}", &lang, &attack.id))).finish()
}

#[get("/{lang}/delete_attack/{attack_id}")]
pub async fn delete_attack(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, attack_id) = path.into_inner();

    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let _attack = Attack::delete(attack_id).expect("Unable to delete attack");

    HttpResponse::Ok().body("")
}