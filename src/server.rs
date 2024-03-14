use actix_web::{App, HttpServer};
use actix_files as fs;

use crate::controllers::get_controller;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_controller::index)
            .service(get_controller::whatsnew)
            .service(get_controller::isaacnews)
            .service(get_controller::isaacyoutube)
            //static files
            .service(fs::Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
