use actix_web::{App, web, HttpServer};
use actix_files as fs;

use crate::controllers::{get_controller, post_controller};
use crate::models::app_state::AppState;


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(get_controller::index)
            .service(get_controller::whatsnew)
            .service(get_controller::isaacnews)
            .service(get_controller::isaacyoutube)
            //api routes (get)
            .service(get_controller::get_app_state)
            .service(get_controller::next_char)
            //api routes (post)
            .service(post_controller::profile_search)
            //static files
            .service(fs::Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
