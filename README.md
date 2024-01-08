# Earthdawn Creatures

This repo is an app to help GMs run the Earthdawn 4e RPG. It allows people to create, edit and update any type of earthdawn creature or NPC and then provides some helpful tracking tools for in-game play.

- [x] Actix-Web w/ async
- [x] Tera for templates
- [x] Diesel accessing Postgresql DB
- [x] User models
- [x] Automated Admin Generation
- [x] Authentication and sign-in
- [x] Email verification and reset password
- [x] Static files
- [x] Fluent integration for i18n

## Dependencies
* Diesel-cli

## Setup
* Clone the repository
* Create `.env` file with the following environmental variables:
    * COOKIE_SECRET_KEY=MINIMUM32CHARACTERS
    * DATABASE_URL
    * SENDGRID_API_KEY=YOUR_API_KEY
    * ADMIN_NAME="YOUR NAME"
    * ADMIN_EMAIL=your@email.com
    * ADMIN_PASSWORD=MINIMUM12CHARACTERS
    * ENVIRONMENT=test
* Change APP_NAME const in lib.rs to your app
* `diesel migration run`
* `cargo run`

# Dan's notes for docker deployment

docker compose down; sleep 2; docker compose up -d db; sleep 10; diesel migration run
docker compose exec -it db psql -U christopherallison -W people_data_api
docker compose logs -f

time docker compose build people-data-api
docker images | grep epi
docker compose up
