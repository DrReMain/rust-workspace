use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

use routers::*;
use state::AppState;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm Ok.".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
