use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest, ResponseError};
use actix_identity::Identity;
use inflector::Inflector;

use crate::{errors::CustomError, generate_basic_context, generate_unique_code, handlers::{CreatureForm, CopyWithMaskForm, DeleteForm}, models::{Attack, InsertableAttack, InsertablePower, InsertableTalent, Locales, Maneuver, Mask, MaskAttack, MaskPower, MaskTalent, MaskManeuver, MaskRef, Power, Tags, Talent, User, UserRole}, AppData};
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

    let masks = Mask::get_all().unwrap_or_default();
    ctx.insert("masks", &masks);

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

    let r_talents = Talent::get_by_creature_id(creature.id);


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

    if let Ok(data) = r_talents {
        ctx.insert("talents", &data);
    }

    let applied_masks: Vec<MaskRef> = creature.masks.iter()
        .filter_map(|mask_id| {
            Mask::get_by_id(&mask_id.expect("Unable to retrieve mask")).ok().map(|m| MaskRef { id: m.id, name: m.name })
        })
        .collect();
    ctx.insert("applied_masks", &applied_masks);

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

    let mut tags = Vec::new();

    if form.creature != None { tags.push(Some(Tags::Creature));};
    if form.spirit != None { tags.push(Some(Tags::Spirit));};
    if form.elemental != None { tags.push(Some(Tags::Elemental));};
    if form.horror != None { tags.push(Some(Tags::Horror));};
    if form.dragon != None { tags.push(Some(Tags::Dragon));};
    if form.horror_construct != None { tags.push(Some(Tags::HorrorConstruct));};
    if form.adept != None { tags.push(Some(Tags::Adept));};
    if form.npc != None { tags.push(Some(Tags::NPC));};
    if form.other != None { tags.push(Some(Tags::Other));};

    // If a mask is selected, apply its stat deltas to the form values
    let mask = if let Some(ref mask_id_str) = form.mask_id {
        if !mask_id_str.is_empty() {
            if let Ok(mask_uuid) = Uuid::parse_str(mask_id_str) {
                Mask::get_by_id(&mask_uuid).ok()
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let (dex, str_, con, per, wil, cha, ini, pd, md, sd, pa, ma,
         ur, dr, wound, kd, actions, rr, karma, circle) = if let Some(ref m) = mask {
        (
            form.dexterity + m.dexterity,
            form.strength + m.strength,
            form.constitution + m.constitution,
            form.perception + m.perception,
            form.willpower + m.willpower,
            form.charisma + m.charisma,
            form.initiative + m.initiative,
            form.pd + m.pd,
            form.md + m.md,
            form.sd + m.sd,
            form.pa + m.pa,
            form.ma + m.ma,
            form.unconsciousness_rating + m.unconsciousness_rating,
            form.death_rating + m.death_rating,
            form.wound + m.wound,
            form.knockdown + m.knockdown,
            form.actions + m.actions,
            form.recovery_rolls + m.recovery_rolls,
            form.karma + m.karma,
            form.circle_rank + m.circle_rank,
        )
    } else {
        (
            form.dexterity, form.strength, form.constitution, form.perception,
            form.willpower, form.charisma, form.initiative, form.pd, form.md,
            form.sd, form.pa, form.ma, form.unconsciousness_rating,
            form.death_rating, form.wound, form.knockdown, form.actions,
            form.recovery_rolls, form.karma, form.circle_rank,
        )
    };

    let new_creature = InsertableCreature::new(
        user.id,
        user.slug.clone(),
        form.name.to_owned(),
        found_in,
        form.rarity.to_owned(),
        circle,
        form.description.to_owned(),
        dex, str_, con, per, wil, cha, ini, pd, md, sd, pa, ma,
        ur, dr, wound, kd, actions,
        form.movement.to_owned(),
        rr, karma,
        tags,
        vec![],
    );

    let creature = Creature::create(&new_creature).expect("Unable to create creature");

    // Apply mask items to the new creature
    if let Some(ref m) = mask {
        if m.attack_action_mod != 0 || m.attack_effect_mod != 0 {
            let empowered = format!("Empowered by {}", m.name);
            if let Ok(mut existing_attacks) = Attack::get_by_creature_id(creature.id) {
                for attack in &mut existing_attacks {
                    attack.action_step += m.attack_action_mod;
                    attack.effect_step += m.attack_effect_mod;
                    attack.details = Some(match &attack.details {
                        Some(d) => format!("{}. {}", d, empowered),
                        None => empowered.clone(),
                    });
                    attack.update().expect("Unable to update attack with mask");
                }
            }
        }
        if let Ok(mask_attacks) = MaskAttack::get_by_mask_id(m.id) {
            for ma in &mask_attacks {
                let new_attack = InsertableAttack::new(
                    user.id,
                    creature.id,
                    ma.name.clone(),
                    ma.action_step,
                    ma.effect_step,
                    ma.details.clone(),
                );
                Attack::create(&new_attack).expect("Unable to create attack from mask");
            }
        }
        if let Ok(mask_powers) = MaskPower::get_by_mask_id(m.id) {
            for mp in &mask_powers {
                let new_power = InsertablePower::new(
                    user.id,
                    creature.id,
                    mp.name.clone(),
                    mp.action_type,
                    mp.target,
                    mp.resisted_by,
                    mp.action_step,
                    mp.effect_step,
                    mp.details.clone(),
                );
                Power::create(&new_power).expect("Unable to create power from mask");
            }
        }
        if let Ok(mask_talents) = MaskTalent::get_by_mask_id(m.id) {
            for mt in &mask_talents {
                let new_talent = InsertableTalent::new(
                    user.id,
                    creature.id,
                    mt.name.clone(),
                    mt.action_step,
                );
                Talent::create(&new_talent).expect("Unable to create talent from mask");
            }
        }
        if let Ok(mask_maneuvers) = MaskManeuver::get_by_mask_id(m.id) {
            for mm in &mask_maneuvers {
                let new_maneuver = InsertableManeuver::new(
                    user.id,
                    creature.id,
                    mm.name.clone(),
                    mm.source.clone(),
                    mm.details.clone(),
                );
                Maneuver::create(&new_maneuver).expect("Unable to create maneuver from mask");
            }
        }
        Creature::add_mask(&creature.id, m.id).expect("Unable to record mask on creature");
    }

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

    let (mut ctx, session_user, role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let creature = Creature::get_by_id(&creature_id).expect("Unable to retrieve creature");

    if session_user != creature.creator_slug && role != UserRole::Admin {
        // Shouldn't be editing this creature. Redirect to the creature view
        return HttpResponse::Found().append_header(("Location", format!("/{}/creature/{}", &lang, &creature.slug))).finish()
    };

    let r_attacks = Attack::get_by_creature_id(creature.id);

    let r_powers = Power::get_by_creature_id(creature.id);

    let r_maneuvers = Maneuver::get_by_creature_id(creature.id);

    let r_talents = Talent::get_by_creature_id(creature.id);

    ctx.insert("creature", &creature);

    if let Ok(attacks) = r_attacks {
        ctx.insert("attacks", &attacks);
    }

    if let Ok(powers) = r_powers {
        ctx.insert("powers", &powers);
    }

    if let Ok(maneuvers) = r_maneuvers {
        ctx.insert("maneuvers", &maneuvers);
    }

    if let Ok(talents) = r_talents {
        ctx.insert("talents", &talents);
    }

    let masks = Mask::get_all().unwrap_or_default();
    ctx.insert("masks", &masks);

    ctx.insert("steps", &data.steps);

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

    let (mut ctx, session_user, role, _lang) = generate_basic_context(id, &lang, req.uri().path());

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

    if session_user != creature.creator_slug && role != UserRole::Admin {
        // Shouldn't be editing this creature. Redirect to the creature view
        return HttpResponse::Found().append_header(("Location", format!("/{}/creature/{}", &lang, &creature.slug))).finish()
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

    let mut tags = Vec::new();

    if form.creature != None { tags.push(Some(Tags::Creature));};
    if form.spirit != None { tags.push(Some(Tags::Spirit));};
    if form.elemental != None { tags.push(Some(Tags::Elemental));};
    if form.horror != None { tags.push(Some(Tags::Horror));};
    if form.dragon != None { tags.push(Some(Tags::Dragon));};
    if form.horror_construct != None { tags.push(Some(Tags::HorrorConstruct));};
    if form.adept != None { tags.push(Some(Tags::Adept));};
    if form.npc != None { tags.push(Some(Tags::NPC));};
    if form.other != None { tags.push(Some(Tags::Other));};

    
    // If a mask is selected, apply its stat deltas to the form values
    let mask = if let Some(ref mask_id_str) = form.mask_id {
        if !mask_id_str.is_empty() {
            if let Ok(mask_uuid) = Uuid::parse_str(mask_id_str) {
                Mask::get_by_id(&mask_uuid).ok()
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let (dex, str_, con, per, wil, cha, ini, edit_pd, edit_md, edit_sd, edit_pa, edit_ma,
         ur, dr, wound, kd, edit_actions, rr, karma, circle) = if let Some(ref m) = mask {
        (
            form.dexterity + m.dexterity,
            form.strength + m.strength,
            form.constitution + m.constitution,
            form.perception + m.perception,
            form.willpower + m.willpower,
            form.charisma + m.charisma,
            form.initiative + m.initiative,
            form.pd + m.pd,
            form.md + m.md,
            form.sd + m.sd,
            form.pa + m.pa,
            form.ma + m.ma,
            form.unconsciousness_rating + m.unconsciousness_rating,
            form.death_rating + m.death_rating,
            form.wound + m.wound,
            form.knockdown + m.knockdown,
            form.actions + m.actions,
            form.recovery_rolls + m.recovery_rolls,
            form.karma + m.karma,
            form.circle_rank + m.circle_rank,
        )
    } else {
        (
            form.dexterity, form.strength, form.constitution, form.perception,
            form.willpower, form.charisma, form.initiative, form.pd, form.md,
            form.sd, form.pa, form.ma, form.unconsciousness_rating,
            form.death_rating, form.wound, form.knockdown, form.actions,
            form.recovery_rolls, form.karma, form.circle_rank,
        )
    };

    let mut our_creature = Creature {
        id: creature.id,
        creator_id: user.id,
        creator_slug: creature.creator_slug,
        name: form.name.to_owned(),
        found_in,
        rarity: form.rarity.to_owned(),
        circle_rank: circle,
        description: form.description.to_owned(),
        dexterity: dex,
        strength: str_,
        constitution: con,
        perception: per,
        willpower: wil,
        charisma: cha,
        initiative: ini,
        pd: edit_pd,
        md: edit_md,
        sd: edit_sd,
        pa: edit_pa,
        ma: edit_ma,
        unconsciousness_rating: ur,
        death_rating: dr,
        wound,
        knockdown: kd,
        actions: edit_actions,
        movement: form.movement.to_owned(),
        recovery_rolls: rr,
        karma,
        slug,
        image_url: None,
        created_at: creature.created_at,
        updated_at: today,
        tags,
        masks: creature.masks.clone(),
    };

    let creature = Creature::update(&mut our_creature).expect("Unable to update creature");

    // Apply mask items to the creature
    if let Some(ref m) = mask {
        if m.attack_action_mod != 0 || m.attack_effect_mod != 0 {
            let empowered = format!("Empowered by {}", m.name);
            if let Ok(mut existing_attacks) = Attack::get_by_creature_id(creature.id) {
                for attack in &mut existing_attacks {
                    attack.action_step += m.attack_action_mod;
                    attack.effect_step += m.attack_effect_mod;
                    attack.details = Some(match &attack.details {
                        Some(d) => format!("{}. {}", d, empowered),
                        None => empowered.clone(),
                    });
                    attack.update().expect("Unable to update attack with mask");
                }
            }
        }
        if let Ok(mask_attacks) = MaskAttack::get_by_mask_id(m.id) {
            for ma in &mask_attacks {
                let new_attack = InsertableAttack::new(
                    user.id,
                    creature.id,
                    ma.name.clone(),
                    ma.action_step,
                    ma.effect_step,
                    ma.details.clone(),
                );
                Attack::create(&new_attack).expect("Unable to create attack from mask");
            }
        }
        if let Ok(mask_powers) = MaskPower::get_by_mask_id(m.id) {
            for mp in &mask_powers {
                let new_power = InsertablePower::new(
                    user.id,
                    creature.id,
                    mp.name.clone(),
                    mp.action_type,
                    mp.target,
                    mp.resisted_by,
                    mp.action_step,
                    mp.effect_step,
                    mp.details.clone(),
                );
                Power::create(&new_power).expect("Unable to create power from mask");
            }
        }
        if let Ok(mask_talents) = MaskTalent::get_by_mask_id(m.id) {
            for mt in &mask_talents {
                let new_talent = InsertableTalent::new(
                    user.id,
                    creature.id,
                    mt.name.clone(),
                    mt.action_step,
                );
                Talent::create(&new_talent).expect("Unable to create talent from mask");
            }
        }
        if let Ok(mask_maneuvers) = MaskManeuver::get_by_mask_id(m.id) {
            for mm in &mask_maneuvers {
                let new_maneuver = InsertableManeuver::new(
                    user.id,
                    creature.id,
                    mm.name.clone(),
                    mm.source.clone(),
                    mm.details.clone(),
                );
                Maneuver::create(&new_maneuver).expect("Unable to create maneuver from mask");
            }
        }
        Creature::add_mask(&creature.id, m.id).expect("Unable to record mask on creature");
    }

    println!("Saved!");

    ctx.insert("creature", &creature);

    // Redirect to creature
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/creature/view/{}", &lang, &creature.slug))).finish()
}

#[get("/{lang}/copy_creature/{creature_id}")]
pub async fn copy_creature(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, session_user, role, _lang) = generate_basic_context(id, &lang, req.uri().path());

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

    if session_user != creature.creator_slug && role != UserRole::Admin {
        // Shouldn't be editing this creature. Redirect to the creature view
        return HttpResponse::Found().append_header(("Location", format!("/{}/creature/{}", &lang, &creature.slug))).finish()
    };

    let today = chrono::Utc::now().naive_utc();

    let rand_string = generate_unique_code(8, false);

    let new_name = format!("Copy of {}-{}", creature.name.to_owned(), rand_string.to_owned());

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
        tags: creature.tags,
        masks: creature.masks.clone(),
    };

    let new_creature = Creature::create(&mut our_creature).expect("Unable to create creature");

    let r_attacks = Attack::get_by_creature_id(creature.id);

    let r_powers = Power::get_by_creature_id(creature.id);

    let r_maneuvers = Maneuver::get_by_creature_id(creature.id);

    let r_talents = Talent::get_by_creature_id(creature_id);

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

    if let Ok(data) = r_talents {

        for element in &data {

            let new_el = InsertableTalent::new(
                user.id,
                new_creature.id,
                element.name.to_owned(),
                element.action_step,
            );

            let _new_talent = Talent::create(&new_el)
                .expect("Unable to create attack");
        };
        
        ctx.insert("talents", &data);
    }

    println!("Saved!");

    ctx.insert("creature", &new_creature);

    // Redirect to creature
    return HttpResponse::Found()
        .append_header(("Location", format!("/{}/edit_creature/{}", &lang, &new_creature.id))).finish()
}

#[get("/{lang}/copy_creature_with_mask/{creature_id}")]
pub async fn copy_creature_with_mask(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        println!("{:?}", &e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang))).finish()
    }

    let creature = match Creature::get_by_id(&creature_id) {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", &e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/", &lang))).finish()
        }
    };

    let masks = Mask::get_all().unwrap_or_default();

    ctx.insert("creature", &creature);
    ctx.insert("masks", &masks);

    let rendered = data.tmpl.render("creatures/copy_creature_with_mask.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_copy_creature_with_mask/{creature_id}")]
pub async fn post_copy_creature_with_mask(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<CopyWithMaskForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    let (_ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);

    if let Err(e) = user {
        println!("{:?}", &e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang))).finish()
    }

    let user = user.unwrap();

    let creature = match Creature::get_by_id(&creature_id) {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", &e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/", &lang))).finish()
        }
    };

    // Resolve the selected mask (if any)
    let mask = if let Some(ref mask_id_str) = form.mask_id {
        if !mask_id_str.is_empty() {
            if let Ok(mask_uuid) = Uuid::parse_str(mask_id_str) {
                Mask::get_by_id(&mask_uuid).ok()
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let today = chrono::Utc::now().naive_utc();
    let rand_string = generate_unique_code(8, false);

    // Apply mask stat deltas on top of the base creature stats
    let (dex, str_, con, per, wil, cha, ini, pd, md, sd, pa, ma,
         ur, dr, wound, kd, actions, rr, karma, circle, name) = if let Some(ref m) = mask {
        (
            creature.dexterity + m.dexterity,
            creature.strength + m.strength,
            creature.constitution + m.constitution,
            creature.perception + m.perception,
            creature.willpower + m.willpower,
            creature.charisma + m.charisma,
            creature.initiative + m.initiative,
            creature.pd + m.pd,
            creature.md + m.md,
            creature.sd + m.sd,
            creature.pa + m.pa,
            creature.ma + m.ma,
            creature.unconsciousness_rating + m.unconsciousness_rating,
            creature.death_rating + m.death_rating,
            creature.wound + m.wound,
            creature.knockdown + m.knockdown,
            creature.actions + m.actions,
            creature.recovery_rolls + m.recovery_rolls,
            creature.karma + m.karma,
            creature.circle_rank + m.circle_rank,
            m.name.to_owned() + " " + &creature.name + "-" + &rand_string,
        )
    } else {
        (
            creature.dexterity, creature.strength, creature.constitution,
            creature.perception, creature.willpower, creature.charisma,
            creature.initiative, creature.pd, creature.md, creature.sd,
            creature.pa, creature.ma, creature.unconsciousness_rating,
            creature.death_rating, creature.wound, creature.knockdown,
            creature.actions, creature.recovery_rolls, creature.karma,
            creature.circle_rank, creature.name + "-" + &rand_string,
        )
    };

    let slug = &name.trim().to_snake_case().to_owned();

    let mut new_insertable = InsertableCreature {
        creator_id: user.id,
        creator_slug: user.slug,
        name: name,
        found_in: creature.found_in,
        rarity: creature.rarity.to_owned(),
        circle_rank: circle,
        description: creature.description.to_owned(),
        dexterity: dex,
        strength: str_,
        constitution: con,
        perception: per,
        willpower: wil,
        charisma: cha,
        initiative: ini,
        pd,
        md,
        sd,
        pa,
        ma,
        unconsciousness_rating: ur,
        death_rating: dr,
        wound,
        knockdown: kd,
        actions,
        movement: creature.movement.to_owned(),
        recovery_rolls: rr,
        karma,
        slug: slug.to_owned(),
        image_url: None,
        created_at: today,
        updated_at: today,
        tags: creature.tags,
        masks: creature.masks.clone(),
    };

    let new_creature = Creature::create(&mut new_insertable).expect("Unable to create creature");

    // Copy the original creature's attacks, powers, maneuvers, and talents
    if let Ok(data) = Attack::get_by_creature_id(creature.id) {
        for element in &data {
            let new_el = InsertableAttack::new(
                user.id, new_creature.id,
                element.name.to_owned(), element.action_step, element.effect_step,
                element.details.clone(),
            );
            Attack::create(&new_el).expect("Unable to create attack");
        }
    }

    if let Ok(data) = Power::get_by_creature_id(creature.id) {
        for element in &data {
            let new_el = InsertablePower::new(
                user.id, new_creature.id,
                element.name.to_owned(), element.action_type, element.target,
                element.resisted_by, element.action_step, element.effect_step,
                element.details.clone(),
            );
            Power::create(&new_el).expect("Unable to create power");
        }
    }

    if let Ok(data) = Maneuver::get_by_creature_id(creature.id) {
        for element in &data {
            let new_el = InsertableManeuver::new(
                user.id, new_creature.id,
                element.name.to_owned(), element.source.to_owned(), element.details.to_owned(),
            );
            Maneuver::create(&new_el).expect("Unable to create maneuver");
        }
    }

    if let Ok(data) = Talent::get_by_creature_id(creature.id) {
        for element in &data {
            let new_el = InsertableTalent::new(
                user.id, new_creature.id,
                element.name.to_owned(), element.action_step,
            );
            Talent::create(&new_el).expect("Unable to create talent");
        }
    }

    // Apply mask items on top
    if let Some(ref m) = mask {
        if m.attack_action_mod != 0 || m.attack_effect_mod != 0 {
            let empowered = format!("Empowered by {}", m.name);
            if let Ok(mut existing_attacks) = Attack::get_by_creature_id(new_creature.id) {
                for attack in &mut existing_attacks {
                    attack.action_step += m.attack_action_mod;
                    attack.effect_step += m.attack_effect_mod;
                    attack.details = Some(match &attack.details {
                        Some(d) => format!("{}. {}", d, empowered),
                        None => empowered.clone(),
                    });
                    attack.update().expect("Unable to update attack with mask");
                }
            }
        }
        if let Ok(mask_attacks) = MaskAttack::get_by_mask_id(m.id) {
            for ma in &mask_attacks {
                let new_el = InsertableAttack::new(
                    user.id, new_creature.id,
                    ma.name.clone(), ma.action_step, ma.effect_step, ma.details.clone(),
                );
                Attack::create(&new_el).expect("Unable to create attack from mask");
            }
        }
        if let Ok(mask_powers) = MaskPower::get_by_mask_id(m.id) {
            for mp in &mask_powers {
                let new_el = InsertablePower::new(
                    user.id, new_creature.id,
                    mp.name.clone(), mp.action_type, mp.target, mp.resisted_by,
                    mp.action_step, mp.effect_step, mp.details.clone(),
                );
                Power::create(&new_el).expect("Unable to create power from mask");
            }
        }
        if let Ok(mask_talents) = MaskTalent::get_by_mask_id(m.id) {
            for mt in &mask_talents {
                let new_el = InsertableTalent::new(
                    user.id, new_creature.id,
                    mt.name.clone(), mt.action_step,
                );
                Talent::create(&new_el).expect("Unable to create talent from mask");
            }
        }
        if let Ok(mask_maneuvers) = MaskManeuver::get_by_mask_id(m.id) {
            for mm in &mask_maneuvers {
                let new_el = InsertableManeuver::new(
                    user.id, new_creature.id,
                    mm.name.clone(), mm.source.clone(), mm.details.clone(),
                );
                Maneuver::create(&new_el).expect("Unable to create maneuver from mask");
            }
        }
        Creature::add_mask(&new_creature.id, m.id).expect("Unable to record mask on creature");
    }

    println!("Saved!");

    HttpResponse::Found()
        .append_header(("Location", format!("/{}/edit_creature/{}", &lang, &new_creature.id))).finish()
}

#[get("/{lang}/delete_creature/{id}")]
pub async fn delete_creature_handler(
    path: web::Path<(String, Uuid)>,
    data: web::Data<AppData>,
    
    req: HttpRequest,
    id: Option<Identity>,
) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    if let Some(id) = id {

        let (mut ctx, session_user, role, _lang) = generate_basic_context(Some(id), &lang, req.uri().path());
        
        let result = Creature::get_by_id(&creature_id);

        let creature = match result {
            Ok(c) => c,
            Err(e) => {
                // Unable to retrieve creature
                println!("{:?}", &e);
                // validate form has data or and permissions exist
                return HttpResponse::Found().append_header(("Location", format!("/{}", &lang))).finish()
            }
        };
        
        if session_user != creature.creator_slug && role != UserRole::Admin {
            println!("User doesn't have rights to delete");
            HttpResponse::Found().append_header(("Location", "/{{ lang }}/creature/view/{{ creature.slug }}")).finish()
        } else {
            ctx.insert("creature", &creature);

            // Handle Creature Delete
        
            let rendered = data.tmpl.render("creatures/delete_creature.html", &ctx).unwrap();
            return HttpResponse::Ok().body(rendered)
                
            }
        } else {
            // Redirect to Login
            return HttpResponse::Found().append_header(("Location", format!("/{}/log_in", &lang))).finish()
        }
}


#[post("/{lang}/delete_creature/{creature_id}")]
pub async fn delete_creature(
    path: web::Path<(String, Uuid)>,
    _data: web::Data<AppData>,
    req: HttpRequest,
    id: Option<Identity>,
    form: web::Form<DeleteForm>,
) -> impl Responder {

    let (lang, creature_id) = path.into_inner();

    if let Some(id) = id {

        let (_ctx, session_user, role, _lang) = generate_basic_context(Some(id), &lang, req.uri().path());
        
        let creature = Creature::get_by_id(&creature_id);
        
        match creature {
            Ok(c) => {

                if session_user != c.creator_slug && role != UserRole::Admin {
                    let err = CustomError::new(
                        406,
                        "Not authorized".to_string(),
                    );
                    println!("{}", &err);
                    return err.error_response()
                } else {
                    if form.verify.trim().to_string() == c.name {
                        println!("Creature matches verify string - deleting");

                        // Delete attacks, powers and maneuvers
                        // Should be handled by ON DELETE CASCADE in PSQL                      
                        
                        // delete creature
                        Creature::delete(c.id).expect("Unable to delete creature");
                        return HttpResponse::Found().append_header(("Location", format!("/{}", &lang))).finish()
                    } else {
                        println!("Creature does not match verify string - return to delete page");
                        return HttpResponse::Found().append_header(("Location", format!("/{}/delete_creature/{}", &lang, c.id))).finish()
                        
                    };
                };
            },
                    
            Err(err) => {
                // no creature returned for ID
                println!("{}", err);
                return err.error_response()
            },
        }
    } else {
        // Redirect to Login
        return HttpResponse::Found().append_header(("Location", format!("/{}/log_in", &lang))).finish()
    }
}
