use actix_web::{App, web, HttpServer};
use actix_files as fs;

use crate::controllers::{get_controller, post_controller};
use crate::models::app_data::AppData;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {

    let app_data = web::Data::new(AppData::new().await);

    HttpServer::new(move|| {
        App::new()
            .app_data(app_data.clone()) 
            .service(get_controller::index)
            .service(get_controller::dashboard)
            .service(get_controller::whatsnew)
            .service(get_controller::isaacnews)
            .service(get_controller::isaacyoutube)
            .service(get_controller::contact)
            .service(get_controller::faqs)
            //api routes (get)
            .service(get_controller::api_isaacyoutubers)
            //api routes (post)
            .service(post_controller::profile_search)
            //static files
            .service(fs::Files::new("/static", "./static"))
            .default_service(web::to(get_controller::not_found))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
