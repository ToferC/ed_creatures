use actix_web::{web, get, Responder, HttpResponse, HttpRequest};
use actix_identity::Identity;

use crate::{generate_basic_context, AppData};

use crate::models::{ToDo, ToDoList};

#[get("/")]
pub async fn raw_index() -> impl Responder {
    return HttpResponse::Found().append_header(("Location", "/en")).finish()
}

#[get("/{lang}")]
pub async fn index(
    data: web::Data<AppData>,
    path: web::Path<String>,

    id: Option<Identity>,
    req: HttpRequest,
) -> impl Responder {

    let lang = path.into_inner();

    let (mut ctx, _, _, _) = generate_basic_context(id, &lang, req.uri().path());

    let todo_lists = ToDoList::get_all().expect("Unable to get lists");

    let mut todos: Vec<ToDo> = Vec::new();

    for list in todo_lists {
        let mut td = ToDoList::get_active_todos(list.id).unwrap();
        todos.append(&mut td);
    }

    ctx.insert("todos", &todos);

    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)

}