mod todolist;

use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use todolist::services;
struct AppState {
    todo_entries: Mutex<Vec<TodoEntry>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct TodoEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "this is a health check route".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todo_entries: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    }).bind(("127.0.0.1", 8080))?.run().await
}
