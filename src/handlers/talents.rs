use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData, models::{User, Talent, InsertableTalent}, handlers::TalentForm};
use uuid::Uuid;

#[get("/{lang}/talents/{creature_id}")]
pub async fn get_talents(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let talents = Talent::get_by_creature_id(creature_id).expect("Unable to retrieve talent");

    ctx.insert("talents", &talents);
    ctx.insert("creature_id", &creature_id);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("talents/talents.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/talent/{talent_id}")]
pub async fn get_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, talent_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let talent = Talent::get_by_id(&talent_id).expect("Unable to retrieve talent");

    ctx.insert("talent", &talent);
    ctx.insert("creature_id", &talent.creature_id);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("talents/talent.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/add_talent/{creature_id}")]
pub async fn add_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("creature_id", &creature_id);

    let rendered = data.tmpl.render("talents/add_talent.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_talent/{creature_id}")]
pub async fn post_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<TalentForm>,
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

    let new_talent = InsertableTalent::new(
        user.id,
        creature_id,
        form.name.to_owned(),
        form.action_step,
        );

    let _talent = Talent::create(&new_talent).expect("Unable to create talent");

    println!("Saved!");

    let result = Talent::get_by_creature_id(creature_id);

    let talents = match result {
        Ok(a) => a,
        Err(e) => {
            // Unable to retrieve talents
            // validate form has data or and permissions exist
            println!("{:?}", e);
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &creature_id))).finish()
        }
    };


    ctx.insert("talents", &talents);
    ctx.insert("creature_id", &creature_id);
    ctx.insert("steps", &data.steps);

    //Redirect to get talent with talent slug
    let rendered = data.tmpl.render("talents/talents.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_talent/{talent_id}")]
pub async fn edit_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, talent_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let talent = Talent::get_by_id(&talent_id).expect("Unable to retrieve talent");

    ctx.insert("talent", &talent);

    let rendered = data.tmpl.render("talents/edit_talent_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_talent_post/{talent_id}")]
pub async fn edit_talent_post(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<TalentForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, talent_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user).expect("Unable to retrive user");

    let result = Talent::get_by_id(&talent_id);

    let talent = match result {
        Ok(c) => c,
        Err(_e) => {
            // Unable to retrieve talent
            // validate form has data or and permissions exist
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_talent/{}", &lang, &talent_id))).finish()
        }
    };

    let today = chrono::Utc::now().naive_utc();

    let mut our_talent = Talent {
        id: talent.id,
        creator_id: user.id,
        creature_id: talent.creature_id,
        name: form.name.to_owned(),
        action_step: form.action_step,
        created_at: talent.created_at,
        updated_at: today,
    };

    let talent = Talent::update(&mut our_talent).expect("Unable to create talent");

    println!("Saved!");

    ctx.insert("talent", &talent);

    // Redirect to talent
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/talent/{}", &lang, &talent.id))).finish()
}

#[get("/{lang}/delete_talent/{talent_id}")]
pub async fn delete_talent(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, talent_id) = path.into_inner();

    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let _talent = Talent::delete(talent_id).expect("Unable to delete talent");

    HttpResponse::Ok().body("")
}