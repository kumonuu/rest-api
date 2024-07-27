use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use actix_web::web::Json;
use crate::{AppState, TodoEntry};
use super::models::{CreateEntryData, UpdateEntryData};

#[get("/todo/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todo_entries.lock().unwrap().to_vec())
}

#[post("/todo/entries")]
async fn create_entry(
    data: web::Data<AppState>, param_obj: Json<CreateEntryData>
) -> impl Responder {
    let mut todo_entries = data.todo_entries.lock().unwrap();
    let mut max_id: i32 = 0;
    for i in 0..todo_entries.len() {
        if todo_entries[i].id > max_id {
            max_id = todo_entries[i].id;
        }
    }
    todo_entries.push(TodoEntry {
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });
    HttpResponse::Ok().json(todo_entries.to_vec())
}

#[put("/todo/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: Json<UpdateEntryData>
) -> impl Responder {
    let id = path.into_inner();
    let mut todo_entries = data.todo_entries.lock().unwrap();
    for i in 0..todo_entries.len() {
        if todo_entries[i].id == id {
            todo_entries[i].title = param_obj.title.clone();
            break;
        }
    }
    HttpResponse::Ok().json(todo_entries.to_vec())
}

#[delete("/todo/entries/{id}")]
async fn delete_entry(
    data: web::Data<AppState>, path: web::Path<i32>
) -> impl Responder {
    let mut todo_entries = data.todo_entries.lock().unwrap();
    let id = path.into_inner();
    *todo_entries = todo_entries.to_vec().into_iter().filter(|x| x.id != id).collect();
    HttpResponse::Ok().json(todo_entries.to_vec())
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}