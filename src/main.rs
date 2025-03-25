use actix_web::{App, HttpServer, web};

// import controller and schema crates to use services and structs defined.
use rust_crud_apis::controller::user_controller::{create_user, get_users};
use rust_crud_apis::schema::{db_schema::UserDb, user_schema::User};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// use macro to make the main function asynchronous
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "localhost";
    let port = 8080;

    // Runtime Database Reference.
    // Arc<Mutex<HashMap>>: Mutable Smart Pointer on HashMap.
    // HashMap contains two fields -> Key : Value -> key = u32 : value = User type
    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    // create new server using the actix_web::HttpServer crate
    // bind to the host and port.
    // define the number of threads (workers)
    // execute the server to listen request and send response.
    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());

        App::new()
            .app_data(app_data)
            .service(get_users)
            .service(create_user)
    })
    .bind(format!("{}:{}", host, port))?
    .workers(3)
    .run()
    .await
}
