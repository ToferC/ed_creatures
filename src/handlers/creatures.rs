use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;
use inflector::Inflector;

use crate::{generate_basic_context, AppData, models::{User, Attack, Power, Locales, Maneuver, InsertableAttack, InsertablePower}, handlers::CreatureForm};
use uuid::Uuid;

use crate::models::{Creature, InsertableCreature, InsertableManeuver};

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
        println!("No user found: {:?}. Redirecting", &e);
        return HttpResponse::Found()
        .append_header(("Location", format!("/{}/", &lang))).finish()
    };

    let user = user.unwrap();

    let creature = InsertableCreature::default(user.id, session_user);

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

    ctx.insert("current_damage", &0);
    ctx.insert("current_wounds", &0);
    
    let rendered = match view.as_str() {
        "in_game" => data.tmpl.render("creatures/in_game_creature.html", &ctx).unwrap(),
        _ => data.tmpl.render("creatures/creature.html", &ctx).unwrap(),
    };

    
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_creature")]
pub async fn post_creature(
    _data: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Form<CreatureForm>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());
    
    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        // no user found so redirect to home
        println!("{:?}", &e);
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
        user.slug,
        form.name.to_owned(),
        found_in,
        form.rarity.to_owned(),
        form.circle_rank,
        form.description.to_owned(),
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
    _data: web::Data<AppData>,
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
        Err(e) => {
            // Unable to retrieve creature
            println!("{:?}", &e);
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
        creator_slug: creature.creator_slug,
        name: form.name.to_owned(),
        found_in: found_in,
        rarity: form.rarity.to_owned(),
        circle_rank: form.circle_rank,
        description: form.description.to_owned(),
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

#[post("/{lang}/copy_creature/{creature_id}")]
pub async fn copy_creature_post(
    _data: web::Data<AppData>,
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
        Err(e) => {
            // Unable to retrieve creature
            println!("{:?}", &e);
            // validate form has data or and permissions exist
            return HttpResponse::Found().append_header(("Location", format!("/{}/edit_creature/{}", &lang, &creature_id))).finish()
        }
    };

    let today = chrono::Utc::now().naive_utc();

    let new_name = format!("Copy of {}", creature.name.to_owned());

    let slug = new_name.trim().to_snake_case();

    let mut our_creature = InsertableCreature {
        creator_id: user.id,
        creator_slug: user.slug,
        name: new_name,
        found_in: creature.found_in,
        rarity: creature.rarity.to_owned(),
        circle_rank: creature.circle_rank,
        description: creature.description.to_owned(),
        dexterity: creature.dexterity,
        strength: creature.strength,
        constitution: creature.constitution,
        perception: creature.perception,
        willpower: creature.willpower,
        charisma: creature.charisma,
        initiative: creature.initiative,
        pd: creature.pd,
        md: creature.md,
        sd: creature.sd,
        pa: creature.pa,
        ma: creature.ma,
        unconsciousness_rating: creature.unconsciousness_rating,
        death_rating: creature.death_rating,
        wound: creature.wound,
        knockdown: creature.knockdown,
        actions: creature.actions,
        movement: creature.movement.to_owned(),
        recovery_rolls: creature.recovery_rolls,
        karma: creature.karma,
        slug,
        image_url: None,
        created_at: today,
        updated_at: today,
    };

    let new_creature = Creature::create(&mut our_creature).expect("Unable to create creature");

    let r_attacks = Attack::get_by_creature_id(creature.id);

    let r_powers = Power::get_by_creature_id(creature.id);

    let r_maneuvers = Maneuver::get_by_creature_id(creature.id);

    if let Ok(data) = r_attacks {

        for element in &data {

            let details = match &element.details {
                Some(s) => Some(s.to_owned()),
                None => None,
            };

            let new_el = InsertableAttack::new(
                user.id,
                new_creature.id,
                element.name.to_owned(),
                element.action_step,
                element.effect_step,
                details,
            );

            let _new_attack = Attack::create(&new_el)
                .expect("Unable to create attack");
        };
        
        ctx.insert("attacks", &data);
    }

    if let Ok(data) = r_powers {

        for element in &data {

            let details = match &element.details {
                Some(s) => Some(s.to_owned()),
                None => None,
            };

            let new_el = InsertablePower::new(
                user.id,
                new_creature.id,
                element.name.to_owned(),
                element.action_type,
                element.target,
                element.resisted_by,
                element.action_step,
                element.effect_step,
                details,
            );

            let _new_power = Power::create(&new_el)
                .expect("Unable to create power");
        };
        
        ctx.insert("powers", &data);
    }

    if let Ok(data) = r_maneuvers {

        for element in &data {
            let new_el = InsertableManeuver::new(
                user.id,
                new_creature.id,
                element.name.to_owned(),
                element.source.to_owned(),
                element.details.to_owned(),
            );

            let _new_maneuver = Maneuver::create(&new_el)
                .expect("Unable to create maneuver");
        }
        ctx.insert("maneuvers", &data);
    }

    println!("Saved!");

    ctx.insert("creature", &new_creature);

    // Redirect to creature
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/creature/edit_creature/{}", &lang, &new_creature.id))).finish()
}