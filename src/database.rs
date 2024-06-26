use crate::{errors::CustomError, models::{Attack, Creature, InsertableAttack, InsertableCreature, InsertablePower, InsertableTalent, Power, Talent}};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;
use crate::models::{User, UserData};
use uuid::Uuid;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create DB pool")
    };
}

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get DB connection");
    run_migration(&mut conn);

    // Auto-add admin if does not exist
    let admin_name = env::var("ADMIN_NAME").expect("Unable to load admin name");
    let admin_email = env::var("ADMIN_EMAIL").expect("Unable to load admin email");
    let admin_pwd = env::var("ADMIN_PASSWORD").expect("Unable to load admin password");
    
    let admin = User::find_from_email(&admin_email);

    match admin {
        // Checking admin and if not, add default data structures
        Ok(u) => println!("Admin exists {:?} - bypass setup", &u),
        Err(_e) => {

            let admin_data = UserData {
                user_name: admin_name.trim().to_owned(),
                email: admin_email.trim().to_owned(),
                password: admin_pwd.trim().to_owned(),
                validated: true,
                role: crate::models::UserRole::Admin,
            };
        
            let admin = User::create(admin_data)
                .expect("Unable to create admin");
        
            println!("Admin created: {:?}", &admin);

            let environment = env::var("ENVIRONMENT");
            
            let environment = match environment {
                Ok(v) => v,
                Err(_) => String::from("test"),
            };

            if environment != "production" {
                // Create user and pre-populate creatures on test
                let user_data = UserData {
                    user_name: "Some Person".to_owned(),
                    email: "someone@email.com".to_owned(),
                    password: "WOOLYHIPPOSOUNDFILE".to_owned(),
                    validated: true,
                    role: crate::models::UserRole::User,
                };
            
                let user = User::create(user_data)
                    .expect("Unable to create user");
            
                println!("User created: {:?}", &admin);
    
                let _r = pre_populate_creatures(user.id, user.slug);
            }
        }
    }
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting DB connection: {}", e)))
}

pub fn pre_populate_creatures(user_id: Uuid, user_slug: String) -> Result<(), CustomError> {
    
    let mut c1 = InsertableCreature::default(user_id, user_slug.to_owned());

    c1.name = "Esparaga".to_string();
    c1.slug = "espagra".to_string();
    
    let r1 = Creature::get_or_create(&c1)?;

    let a1 = InsertableAttack::default(user_id, r1.id);

    Attack::create(&a1).expect("Unable to create attack");

    let p1 = InsertablePower::default(user_id, r1.id);

    Power::create(&p1).expect("Unable to create attack");

    let mut c2 = InsertableCreature::default(user_id, user_slug.to_owned());

    c2.name = "Ghoul".to_string();
    c2.slug = "ghoul".to_string();
    c2.circle_rank = 2;

    Creature::get_or_create(&c2)?;

    let mut c3 = InsertableCreature::default(user_id, user_slug);

    c3.name = "Dwarf Weaponsmith".to_string();
    c3.strength = 6;
    c3.perception = 6;
    c3.constitution = 5;
    c3.dexterity = 6;
    c3.willpower = 5;
    c3.charisma = 6;
    c3.slug = "dwarf_weaponsmith".to_string();
    c3.circle_rank = 3;

    let dwarf = Creature::get_or_create(&c3)?;

    let talent1 = InsertableTalent::new(user_id, dwarf.id, "Forge Weapon".to_owned(), 9);
    Talent::create(&talent1).expect("Unable to create talent");

    let talent2 = InsertableTalent::new(user_id, dwarf.id, "Awareness".to_owned(), 8);
    Talent::create(&talent2).expect("Unable to create talent");

    Ok(())
}