use actix_web::{get, HttpResponse, post, web};
use actix_web::web::ServiceConfig;
use uuid::Uuid;
use crate::data::{AppState, ToDo};
use chrono::Utc;

#[post("/todos")]
async fn create(mut todo: web::Json<ToDo>, app_state: web::Data<AppState>) -> HttpResponse{
    let id = Uuid::new_v4().to_string();
    todo.id = Some(id.clone());
    todo.create_time = Some(Utc::now());
    todo.completed = Some(false);

    if let Ok(mut todos) = app_state.todos.lock(){
        todos.push(todo.clone());
    }

    HttpResponse::Created()
        .append_header(("Location", format!("http://localhost::8080/api/todos{}", id)))
        .json(todo)
}

#[get("/todos/{id}")]
async fn get_by_id(path: web::Path<String>, app_state: web::Data<AppState>) -> HttpResponse{
    let id = Some(path.to_string());
    if let Ok(todos) = app_state.todos.lock(){
        let todo = todos.iter()
            .find(|todo| todo.id == id);

        if todo.is_some() {
            HttpResponse::Ok().json(todo.unwrap())
        }
        else {
            HttpResponse::NotFound().finish()
        }
    }
    else{
        HttpResponse::InternalServerError().finish()
    }

}

pub fn config(config: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        .service(create)
        .service(get_by_id);

    config.service(scope);
}