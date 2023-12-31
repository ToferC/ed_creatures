use crate::{errors::CustomError, models::InsertableCreature};
use chrono::Duration;
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
                role: "admin".to_owned(),
            };
        
            let admin = User::create(admin_data)
                .expect("Unable to create admin");
        
            println!("Admin created: {:?}", &admin);

            // Create user and pre-populate to-dos

            let user_data = UserData {
                user_name: "Some Person".to_owned(),
                email: "someone@email.com".to_owned(),
                password: "WOOLYHIPPOSOUNDFILE".to_owned(),
                validated: true,
                role: "user".to_owned(),
            };
        
            let user = User::create(user_data)
                .expect("Unable to create user");
        
            println!("User created: {:?}", &admin);

            let r = pre_populate_creatures(user.id);
        }
    }
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting DB connection: {}", e)))
}

pub fn pre_populate_creatures(user_id: Uuid) -> Result<(), CustomError> {
    

    Ok(())
}