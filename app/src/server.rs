use actix_web::{App, web, HttpServer};
use actix_files as fs;

use crate::controllers::{get_controller, post_controller};

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(get_controller::index)
            .service(get_controller::dashboard)
            .service(get_controller::whatsnew)
            .service(get_controller::isaacnews)
            .service(get_controller::isaacyoutube)
            .service(get_controller::contact)
            .service(get_controller::faqs)
            //api routes (post)
            .service(post_controller::profile_search)
            //static files
            .service(fs::Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
