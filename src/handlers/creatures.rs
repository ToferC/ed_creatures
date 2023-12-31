use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData};
use uuid::Uuid;

use crate::models::{Creature, InsertableCreature};

#[get("/{lang}/new_creature_form")]
pub async fn new_creature_form(
    data: web::Data<AppData>,
    path: web::Path<String>,

    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());

    let rendered = data.tmpl.render("new_creature_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)

}

#[get("/{lang}/creature/{slug}")]
pub async fn get_creature(
    data: web::Data<AppData>,
    path: web::Path<(String, String)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, slug) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let creature = Creature::get_by_slug(slug).expect("Unable to retrieve creature");

    ctx.insert("creature", &creature);

    let rendered = data.tmpl.render("creature.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_creature")]
pub async fn post_creature(
    data: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Form<Insertablecreature>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_by_slug(session_user).expect("Unable to find user");

    let new_creature = InsertableCreature::new(
        user.id,
        form.creature_name.to_owned(),
        form.found_in.to_owned(),
        form.rarity.to_owned(),
        form.circle_rank,
        form.dex,
        form.strength,
        form.con,
        form.per,
        form.wil,
        form.cha,
        form.initiative,
        form.pd,
        form.md,
        form.sd,
        form.pa,
        form.ma,
        form.unconscious_rating,
        form.death_rating,
        form.wound,
        form.knockdown,
        form.actions,
        form.recovery_rolls,
        );

    let creature = Creature::create(&new_creature).expect("Unable to create creature");

    println!("Saved!");

    ctx.insert("creature", &creature);

    // TODO: Redirect to get creature with creature slug
    let rendered = data.tmpl.render("texts/creature.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_creature/{slug}")]
pub async fn edit_creature(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, slug) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let creature = Creature::get_by_slug(slug).expect("Unable to retrieve creature");

    ctx.insert("creature", &creature);

    let rendered = data.tmpl.render("edit_creature.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_creature/{slug}")]
pub async fn edit_creature(
    data: web::Data<AppData>,
    path: web::Path<(String, slug)>,
    form: web::Form<Insertablecreature>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let result = Creature::get_by_slug(creature_id);

    if let result == Ok(c) {
        let creature = c;

        let new_creature = InsertableCreature::new(
            form.creature_name.to_owned(),
            form.found_in.to_owned(),
            form.rarity.to_owned(),
            form.circle_rank,
            form.dex,
            form.strength,
            form.con,
            form.per,
            form.wil,
            form.cha,
            form.initiative,
            form.pd,
            form.md,
            form.sd,
            form.pa,
            form.ma,
            form.unconscious_rating,
            form.death_rating,
            form.wound,
            form.knockdown,
            form.actions,
            form.recovery_rolls,
            );
    
        let creature = Creature::update(&new_creature).expect("Unable to create creature");
    
        println!("Saved!");
    
        ctx.insert("creature", &creature);
    
        // TODO: Redirect to get creature with creature slug
        let rendered = data.tmpl.render("texts/creature.html", &ctx).unwrap();
            HttpResponse::Ok().body(rendered)

    } else {
         // Unable to retrieve creature
         // validate form has data or and permissions exist
         return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &slug))).finish()

    }

    
}