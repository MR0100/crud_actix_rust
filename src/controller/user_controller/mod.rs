use actix_web::{HttpResponse, Responder, error::ErrorNotFound, web};

use crate::schema::{
    db_schema::UserDb,
    user_schema::{CreateUserResponse, User},
};

#[actix_web::get("/users/{id}")]
async fn get_users(user_id: web::Path<u32>, db: web::Data<UserDb>) -> impl Responder {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User Not Found!")),
    }
}

#[actix_web::post("/users")]
async fn create_user(user_data: web::Json<User>, db: web::Data<UserDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0).checked_add(1).unwrap();

    let name = user_data.name.clone();

    db.insert(new_id, user_data.into_inner());

    HttpResponse::Created().json(CreateUserResponse { id: new_id, name })
}
