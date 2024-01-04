use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest, put};
use actix_identity::Identity;
use inflector::Inflector;

use crate::{generate_basic_context, AppData, models::{User, Attack, Power, Locales, Maneuver}, handlers::CreatureForm};
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

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        // no user found so redirect to home
        println!("No user found. Redirecting");
        return HttpResponse::Found()
        .append_header(("Location", format!("/{}/", &lang))).finish()
    };

    let user = user.unwrap();

    let creature = InsertableCreature::default(user.id);

    ctx.insert("creature", &creature);

    let rendered = data.tmpl.render("creatures/new_creature_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)

}

#[get("/{lang}/creature/{view}/{slug}")]
pub async fn get_creature(
    data: web::Data<AppData>,
    path: web::Path<(String, String, String)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, view, slug) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let creature = Creature::get_by_slug(&slug).expect("Unable to retrieve creature");

    let r_attacks = Attack::get_by_creature_id(creature.id);

    let r_powers = Power::get_by_creature_id(creature.id);

    let r_maneuvers = Maneuver::get_by_creature_id(creature.id);

    ctx.insert("creature", &creature);
    ctx.insert("steps", &data.steps);

    if let Ok(data) = r_attacks {
        ctx.insert("attacks", &data);
    }

    if let Ok(data) = r_powers {
        ctx.insert("powers", &data);
    }

    if let Ok(data) = r_maneuvers {
        ctx.insert("maneuvers", &data);
    }

    let rendered = match view.as_str() {
        "in_game" => data.tmpl.render("creatures/in_game_creature.html", &ctx).unwrap(),
        _ => data.tmpl.render("creatures/creature.html", &ctx).unwrap(),
    };
    
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_creature")]
pub async fn post_creature(
    data: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Form<CreatureForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());
    
    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        // no user found so redirect to home
        return HttpResponse::Found()
        .append_header(("Location", format!("/{}/", &lang))).finish()
    }

    let user = user.unwrap();

    let mut found_in = Vec::new();

    if form.jungle != None { found_in.push(Some(Locales::Jungle));};
    if form.desert != None { found_in.push(Some(Locales::Desert));};
    if form.forest != None { found_in.push(Some(Locales::Forest));};
    if form.plains != None { found_in.push(Some(Locales::Plains));};
    if form.urban != None { found_in.push(Some(Locales::Urban));};
    if form.mountain != None { found_in.push(Some(Locales::Mountain));};
    if form.cavern != None { found_in.push(Some(Locales::Cavern));};
    if form.swamp != None { found_in.push(Some(Locales::Swamp));};
    if form.kaer != None { found_in.push(Some(Locales::Kaer));};
    if form.any != None { found_in.push(Some(Locales::Any));};

    let new_creature = InsertableCreature::new(
        user.id,
        form.name.to_owned(),
        found_in,
        form.rarity.to_owned(),
        form.circle_rank,
        form.dexterity,
        form.strength,
        form.constitution,
        form.perception,
        form.willpower,
        form.charisma,
        form.initiative,
        form.pd,
        form.md,
        form.sd,
        form.pa,
        form.ma,
        form.unconsciousness_rating,
        form.death_rating,
        form.wound,
        form.knockdown,
        form.actions,
        form.movement.to_owned(),
        form.recovery_rolls,
        form.karma,
        );

    let creature = Creature::create(&new_creature).expect("Unable to create creature");

    println!("Saved!");

    ctx.insert("creature", &creature);

    //Redirect to get creature with creature slug
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/creature/view/{}", &lang, &creature.slug))).finish()
}

#[get("/{lang}/edit_creature/{creature_id}")]
pub async fn edit_creature(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let creature = Creature::get_by_id(&creature_id).expect("Unable to retrieve creature");

    let r_attacks = Attack::get_by_creature_id(creature.id);

    let r_powers = Power::get_by_creature_id(creature.id);

    let r_maneuvers = Maneuver::get_by_creature_id(creature.id);

    ctx.insert("creature", &creature);

    if let Ok(data) = r_attacks {
        ctx.insert("attacks", &data);
    }

    if let Ok(data) = r_powers {
        ctx.insert("powers", &data);
    }

    if let Ok(data) = r_maneuvers {
        ctx.insert("maneuvers", &data);
    }

    let rendered = data.tmpl.render("creatures/edit_creature_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_creature_post/{creature_id}")]
pub async fn edit_creature_post(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<CreatureForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user).expect("Unable to retrive user");

    let result = Creature::get_by_id(&creature_id);

    let creature = match result {
        Ok(c) => c,
        Err(r) => {
            // Unable to retrieve creature
            // validate form has data or and permissions exist
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &creature_id))).finish()
        }
    };

    let today = chrono::Utc::now().naive_utc();
    let slug = form.name.trim().to_snake_case();

    let mut found_in = Vec::new();

    if form.jungle != None { found_in.push(Some(Locales::Jungle));};
    if form.desert != None { found_in.push(Some(Locales::Desert));};
    if form.forest != None { found_in.push(Some(Locales::Forest));};
    if form.plains != None { found_in.push(Some(Locales::Plains));};
    if form.urban != None { found_in.push(Some(Locales::Urban));};
    if form.mountain != None { found_in.push(Some(Locales::Mountain));};
    if form.cavern != None { found_in.push(Some(Locales::Cavern));};
    if form.swamp != None { found_in.push(Some(Locales::Swamp));};
    if form.kaer != None { found_in.push(Some(Locales::Kaer));};
    if form.any != None { found_in.push(Some(Locales::Any));};

    let mut our_creature = Creature {
        id: creature.id,
        creator_id: user.id,
        name: form.name.to_owned(),
        found_in: found_in,
        rarity: form.rarity.to_owned(),
        circle_rank: form.circle_rank,
        dexterity: form.dexterity,
        strength: form.strength,
        constitution: form.constitution,
        perception: form.perception,
        willpower: form.willpower,
        charisma: form.charisma,
        initiative: form.initiative,
        pd: form.pd,
        md: form.md,
        sd: form.sd,
        pa: form.pa,
        ma: form.ma,
        unconsciousness_rating: form.unconsciousness_rating,
        death_rating: form.death_rating,
        wound: form.wound,
        knockdown: form.knockdown,
        actions: form.actions,
        movement: form.movement.to_owned(),
        recovery_rolls: form.recovery_rolls,
        karma: form.karma,
        slug,
        image_url: None,
        created_at: creature.created_at,
        updated_at: today,
    };

    let creature = Creature::update(&mut our_creature).expect("Unable to create creature");

    println!("Saved!");

    ctx.insert("creature", &creature);

    // Redirect to creature
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/creature/view/{}", &lang, &creature.slug))).finish()
}