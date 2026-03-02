use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;
use uuid::Uuid;

use crate::{
    generate_basic_context,
    AppData,
    handlers::{DeleteForm, MaskForm, MaskAttackForm, MaskPowerForm, MaskTalentForm, MaskManeuverForm},
    models::{
        Mask, InsertableMask,
        MaskAttack, InsertableMaskAttack,
        MaskPower, InsertableMaskPower,
        MaskTalent, InsertableMaskTalent,
        MaskManeuver, InsertableMaskManeuver,
        User, UserRole,
    },
};

// ---- Mask List ----

#[get("/{lang}/masks")]
pub async fn get_masks(
    data: web::Data<AppData>,
    path: web::Path<String>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let lang = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let masks = Mask::get_all().unwrap_or_default();
    ctx.insert("masks", &masks);

    let rendered = data.tmpl.render("masks/masks.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

// ---- View Mask ----

#[get("/{lang}/mask/{mask_id}")]
pub async fn get_mask(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mask = match Mask::get_by_id(&mask_id) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    let r_attacks = MaskAttack::get_by_mask_id(mask.id);
    let r_powers = MaskPower::get_by_mask_id(mask.id);
    let r_talents = MaskTalent::get_by_mask_id(mask.id);
    let r_maneuvers = MaskManeuver::get_by_mask_id(mask.id);

    ctx.insert("mask", &mask);
    ctx.insert("steps", &data.steps);

    if let Ok(data) = r_attacks { ctx.insert("mask_attacks", &data); }
    if let Ok(data) = r_powers { ctx.insert("mask_powers", &data); }
    if let Ok(data) = r_talents { ctx.insert("mask_talents", &data); }
    if let Ok(data) = r_maneuvers { ctx.insert("mask_maneuvers", &data); }

    let rendered = data.tmpl.render("masks/mask.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

// ---- New Mask Form ----

#[get("/{lang}/new_mask_form")]
pub async fn new_mask_form(
    data: web::Data<AppData>,
    path: web::Path<String>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let lang = path.into_inner();
    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);
    if let Err(e) = user {
        println!("No user found: {:?}. Redirecting", &e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang)))
            .finish();
    }

    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/new_mask_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

// ---- Post New Mask ----

#[post("/{lang}/post_mask")]
pub async fn post_mask(
    _data: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Form<MaskForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let lang = path.into_inner();
    let (_ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);
    if let Err(e) = user {
        println!("{:?}", &e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang)))
            .finish();
    }
    let user = user.unwrap();

    let new_mask = InsertableMask::new(
        user.id,
        user.slug,
        form.name.to_owned(),
        form.description.to_owned(),
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
        form.recovery_rolls,
        form.karma,
        form.attack_action_mod,
        form.attack_effect_mod,
    );

    let mask = Mask::create(&new_mask).expect("Unable to create mask");

    HttpResponse::Found()
        .append_header(("Location", format!("/{}/edit_mask/{}", &lang, &mask.id)))
        .finish()
}

// ---- Edit Mask Form ----

#[get("/{lang}/edit_mask/{mask_id}")]
pub async fn edit_mask(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, session_user, role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mask = match Mask::get_by_id(&mask_id) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    if session_user != mask.creator_slug && role != UserRole::Admin {
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/mask/{}", &lang, &mask.id)))
            .finish();
    }

    let r_attacks = MaskAttack::get_by_mask_id(mask.id);
    let r_powers = MaskPower::get_by_mask_id(mask.id);
    let r_talents = MaskTalent::get_by_mask_id(mask.id);
    let r_maneuvers = MaskManeuver::get_by_mask_id(mask.id);

    ctx.insert("mask", &mask);
    ctx.insert("steps", &data.steps);

    if let Ok(data) = r_attacks { ctx.insert("mask_attacks", &data); }
    if let Ok(data) = r_powers { ctx.insert("mask_powers", &data); }
    if let Ok(data) = r_talents { ctx.insert("mask_talents", &data); }
    if let Ok(data) = r_maneuvers { ctx.insert("mask_maneuvers", &data); }

    let rendered = data.tmpl.render("masks/edit_mask_form.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

// ---- Post Edit Mask ----

#[post("/{lang}/edit_mask_post/{mask_id}")]
pub async fn edit_mask_post(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (_ctx, session_user, role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mut mask = match Mask::get_by_id(&mask_id) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    if session_user != mask.creator_slug && role != UserRole::Admin {
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/mask/{}", &lang, &mask.id)))
            .finish();
    }

    mask.name = form.name.to_owned();
    mask.description = form.description.to_owned();
    mask.circle_rank = form.circle_rank;
    mask.dexterity = form.dexterity;
    mask.strength = form.strength;
    mask.constitution = form.constitution;
    mask.perception = form.perception;
    mask.willpower = form.willpower;
    mask.charisma = form.charisma;
    mask.initiative = form.initiative;
    mask.pd = form.pd;
    mask.md = form.md;
    mask.sd = form.sd;
    mask.pa = form.pa;
    mask.ma = form.ma;
    mask.unconsciousness_rating = form.unconsciousness_rating;
    mask.death_rating = form.death_rating;
    mask.wound = form.wound;
    mask.knockdown = form.knockdown;
    mask.actions = form.actions;
    mask.recovery_rolls = form.recovery_rolls;
    mask.karma = form.karma;
    mask.attack_action_mod = form.attack_action_mod;
    mask.attack_effect_mod = form.attack_effect_mod;

    let updated = Mask::update(&mut mask).expect("Unable to update mask");

    HttpResponse::Found()
        .append_header(("Location", format!("/{}/edit_mask/{}", &lang, &updated.id)))
        .finish()
}

// ---- Delete Mask (GET confirmation) ----

#[get("/{lang}/delete_mask/{mask_id}")]
pub async fn delete_mask_handler(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();

    if let Some(id) = id {
        let (mut ctx, session_user, role, _lang) =
            generate_basic_context(Some(id), &lang, req.uri().path());

        let mask = match Mask::get_by_id(&mask_id) {
            Ok(m) => m,
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Found()
                    .append_header(("Location", format!("/{}/masks", &lang)))
                    .finish();
            }
        };

        if session_user != mask.creator_slug && role != UserRole::Admin {
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/mask/{}", &lang, &mask.id)))
                .finish();
        }

        ctx.insert("mask", &mask);
        let rendered = data.tmpl.render("masks/delete_mask.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        HttpResponse::Found()
            .append_header(("Location", format!("/{}/login", &lang)))
            .finish()
    }
}

// ---- Delete Mask (POST) ----

#[post("/{lang}/delete_mask/{mask_id}")]
pub async fn delete_mask(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<DeleteForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();

    if let Some(id) = id {
        let (_ctx, session_user, role, _lang) =
            generate_basic_context(Some(id), &lang, req.uri().path());

        let mask = match Mask::get_by_id(&mask_id) {
            Ok(m) => m,
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Found()
                    .append_header(("Location", format!("/{}/masks", &lang)))
                    .finish();
            }
        };

        if session_user != mask.creator_slug && role != UserRole::Admin {
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/mask/{}", &lang, &mask.id)))
                .finish();
        }

        if form.verify.trim() == mask.name {
            Mask::delete(mask.id).expect("Unable to delete mask");
            HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish()
        } else {
            HttpResponse::Found()
                .append_header(("Location", format!("/{}/delete_mask/{}", &lang, mask.id)))
                .finish()
        }
    } else {
        HttpResponse::Found()
            .append_header(("Location", format!("/{}/login", &lang)))
            .finish()
    }
}

// ---- MaskAttack CRUD ----

#[get("/{lang}/add_mask_attack/{mask_id}")]
pub async fn add_mask_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("mask_id", &mask_id);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/add_mask_attack.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_mask_attack/{mask_id}")]
pub async fn post_mask_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskAttackForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);
    if let Err(e) = user {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang)))
            .finish();
    }
    let user = user.unwrap();

    let new_attack = InsertableMaskAttack::new(
        user.id,
        mask_id,
        form.name.to_owned(),
        form.action_step,
        form.effect_step,
        form.details.clone(),
    );

    MaskAttack::create(&new_attack).expect("Unable to create mask attack");

    let mask_attacks = MaskAttack::get_by_mask_id(mask_id).unwrap_or_default();
    ctx.insert("mask_id", &mask_id);
    ctx.insert("mask_attacks", &mask_attacks);

    let rendered = data.tmpl.render("masks/mask_attacks.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_mask_attack/{attack_id}")]
pub async fn edit_mask_attack(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, attack_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let attack = match MaskAttack::get_by_id(&attack_id) {
        Ok(a) => a,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    ctx.insert("mask_attack", &attack);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/edit_mask_attack.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_mask_attack_post/{attack_id}")]
pub async fn edit_mask_attack_post(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskAttackForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, attack_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mut attack = match MaskAttack::get_by_id(&attack_id) {
        Ok(a) => a,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    attack.name = form.name.to_owned();
    attack.action_step = form.action_step;
    attack.effect_step = form.effect_step;
    attack.details = form.details.clone();

    let updated = MaskAttack::update(&mut attack).expect("Unable to update mask attack");

    ctx.insert("mask_attack", &updated);

    let rendered = data.tmpl.render("masks/mask_attack.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/delete_mask_attack/{attack_id}")]
pub async fn delete_mask_attack(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, attack_id) = path.into_inner();
    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    if let Err(e) = MaskAttack::get_by_id(&attack_id) {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/masks", &lang)))
            .finish();
    }

    MaskAttack::delete(attack_id).expect("Unable to delete mask attack");

    HttpResponse::Ok().body("")
}

// ---- MaskPower CRUD ----

#[get("/{lang}/add_mask_power/{mask_id}")]
pub async fn add_mask_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("mask_id", &mask_id);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/add_mask_power.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_mask_power/{mask_id}")]
pub async fn post_mask_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskPowerForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);
    if let Err(e) = user {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang)))
            .finish();
    }
    let user = user.unwrap();

    let new_power = InsertableMaskPower::new(
        user.id,
        mask_id,
        form.name.to_owned(),
        form.action_type,
        form.target,
        form.resisted_by,
        form.action_step,
        form.effect_step,
        form.details.clone(),
    );

    MaskPower::create(&new_power).expect("Unable to create mask power");

    let mask_powers = MaskPower::get_by_mask_id(mask_id).unwrap_or_default();
    ctx.insert("mask_id", &mask_id);
    ctx.insert("mask_powers", &mask_powers);

    let rendered = data.tmpl.render("masks/mask_powers.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_mask_power/{power_id}")]
pub async fn edit_mask_power(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, power_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let power = match MaskPower::get_by_id(&power_id) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    ctx.insert("mask_power", &power);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/edit_mask_power.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_mask_power_post/{power_id}")]
pub async fn edit_mask_power_post(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskPowerForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, power_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mut power = match MaskPower::get_by_id(&power_id) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    power.name = form.name.to_owned();
    power.action_type = form.action_type;
    power.target = form.target;
    power.resisted_by = form.resisted_by;
    power.action_step = form.action_step;
    power.effect_step = form.effect_step;
    power.details = form.details.clone();

    let updated = MaskPower::update(&mut power).expect("Unable to update mask power");

    ctx.insert("mask_power", &updated);

    let rendered = data.tmpl.render("masks/mask_power.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/delete_mask_power/{power_id}")]
pub async fn delete_mask_power(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, power_id) = path.into_inner();
    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    if let Err(e) = MaskPower::get_by_id(&power_id) {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/masks", &lang)))
            .finish();
    }

    MaskPower::delete(power_id).expect("Unable to delete mask power");

    HttpResponse::Ok().body("")
}

// ---- MaskTalent CRUD ----

#[get("/{lang}/add_mask_talent/{mask_id}")]
pub async fn add_mask_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("mask_id", &mask_id);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/add_mask_talent.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_mask_talent/{mask_id}")]
pub async fn post_mask_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskTalentForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);
    if let Err(e) = user {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang)))
            .finish();
    }
    let user = user.unwrap();

    let new_talent = InsertableMaskTalent::new(
        user.id,
        mask_id,
        form.name.to_owned(),
        form.action_step,
    );

    MaskTalent::create(&new_talent).expect("Unable to create mask talent");

    let mask_talents = MaskTalent::get_by_mask_id(mask_id).unwrap_or_default();
    ctx.insert("mask_id", &mask_id);
    ctx.insert("mask_talents", &mask_talents);

    let rendered = data.tmpl.render("masks/mask_talents.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_mask_talent/{talent_id}")]
pub async fn edit_mask_talent(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, talent_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let talent = match MaskTalent::get_by_id(&talent_id) {
        Ok(t) => t,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    ctx.insert("mask_talent", &talent);
    ctx.insert("steps", &data.steps);

    let rendered = data.tmpl.render("masks/edit_mask_talent.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_mask_talent_post/{talent_id}")]
pub async fn edit_mask_talent_post(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskTalentForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, talent_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mut talent = match MaskTalent::get_by_id(&talent_id) {
        Ok(t) => t,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    talent.name = form.name.to_owned();
    talent.action_step = form.action_step;

    let updated = MaskTalent::update(&mut talent).expect("Unable to update mask talent");

    ctx.insert("mask_talent", &updated);

    let rendered = data.tmpl.render("masks/mask_talent.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/delete_mask_talent/{talent_id}")]
pub async fn delete_mask_talent(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, talent_id) = path.into_inner();
    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    if let Err(e) = MaskTalent::get_by_id(&talent_id) {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/masks", &lang)))
            .finish();
    }

    MaskTalent::delete(talent_id).expect("Unable to delete mask talent");

    HttpResponse::Ok().body("")
}

// ---- MaskManeuver CRUD ----

#[get("/{lang}/add_mask_maneuver/{mask_id}")]
pub async fn add_mask_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    ctx.insert("mask_id", &mask_id);

    let rendered = data.tmpl.render("masks/add_mask_maneuver.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/post_mask_maneuver/{mask_id}")]
pub async fn post_mask_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskManeuverForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, mask_id) = path.into_inner();
    let (mut ctx, session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let user = User::get_from_slug(&session_user);
    if let Err(e) = user {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/", &lang)))
            .finish();
    }
    let user = user.unwrap();

    let new_maneuver = InsertableMaskManeuver::new(
        user.id,
        mask_id,
        form.name.to_owned(),
        form.source.to_owned(),
        form.details.to_owned(),
    );

    MaskManeuver::create(&new_maneuver).expect("Unable to create mask maneuver");

    let mask_maneuvers = MaskManeuver::get_by_mask_id(mask_id).unwrap_or_default();
    ctx.insert("mask_id", &mask_id);
    ctx.insert("mask_maneuvers", &mask_maneuvers);

    let rendered = data.tmpl.render("masks/mask_maneuvers.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/edit_mask_maneuver/{maneuver_id}")]
pub async fn edit_mask_maneuver(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, maneuver_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let maneuver = match MaskManeuver::get_by_id(&maneuver_id) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    ctx.insert("mask_maneuver", &maneuver);

    let rendered = data.tmpl.render("masks/edit_mask_maneuver.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/edit_mask_maneuver_post/{maneuver_id}")]
pub async fn edit_mask_maneuver_post(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    form: web::Form<MaskManeuverForm>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, maneuver_id) = path.into_inner();
    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let mut maneuver = match MaskManeuver::get_by_id(&maneuver_id) {
        Ok(m) => m,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::Found()
                .append_header(("Location", format!("/{}/masks", &lang)))
                .finish();
        }
    };

    maneuver.name = form.name.to_owned();
    maneuver.source = form.source.to_owned();
    maneuver.details = form.details.to_owned();

    let updated = MaskManeuver::update(&mut maneuver).expect("Unable to update mask maneuver");

    ctx.insert("mask_maneuver", &updated);

    let rendered = data.tmpl.render("masks/mask_maneuver.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/delete_mask_maneuver/{maneuver_id}")]
pub async fn delete_mask_maneuver(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {
    let (lang, maneuver_id) = path.into_inner();
    let (_ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    if let Err(e) = MaskManeuver::get_by_id(&maneuver_id) {
        println!("{:?}", e);
        return HttpResponse::Found()
            .append_header(("Location", format!("/{}/masks", &lang)))
            .finish();
    }

    MaskManeuver::delete(maneuver_id).expect("Unable to delete mask maneuver");

    HttpResponse::Ok().body("")
}
