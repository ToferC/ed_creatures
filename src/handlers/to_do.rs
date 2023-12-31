use actix_web::{web, get, post, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData};
use uuid::Uuid;

use crate::models::{ToDo, InsertableToDo};

#[get("/{lang}/toggle_status/{todo_id}")]
pub async fn toggle_status(
    _data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,

    _id: Option<Identity>,
    _req: HttpRequest,
) -> impl Responder {

    let (lang, todo_id) = path.into_inner();

    let todo = ToDo::get(todo_id).expect("Unable to retrieve todo");

    let _result = todo.toggle_status().expect("Unable to toggle status");

    return HttpResponse::Found().append_header(("Location", format!("/{}", &lang))).finish()

}

#[get("/{lang}/add_todo")]
pub async fn add_todo(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,

    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {

    let (lang, todo_id) = path.into_inner();

    let (mut ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());

    let rendered = data.tmpl.render("add_todo.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)

}

#[get("/{lang}/todo/{todo_id}")]
pub async fn get_text(
    data: web::Data<AppData>,
    path: web::Path<(String, Uuid)>,
    
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let (lang, todo_id) = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let todo = ToDo::get(todo_id).expect("Unable to retrieve todo");

    ctx.insert("todo", &todo);

    let rendered = data.tmpl.render("todo.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[post("/{lang}/create_todo")]
pub async fn create_new_todo(
    data: web::Data<AppData>,
    path: web::Path<String>,
    form: web::Form<InsertableToDo>,
    id: Option<Identity>,
    req:HttpRequest) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, _session_user, _role, _lang) = generate_basic_context(id, &lang, req.uri().path());

    let new_todo = InsertableToDo::new(form.list_id, form.title.to_owned(), form.description.to_owned(), form.priority, form.active);

    let todo = ToDo::create(&new_todo).expect("Unable to create todo");

    println!("Saved!");

    ctx.insert("todo", &todo);

    let rendered = data.tmpl.render("texts/todo.html", &ctx).unwrap();
        HttpResponse::Ok().body(rendered)
}