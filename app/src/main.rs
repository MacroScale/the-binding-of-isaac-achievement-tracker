
use env_logger;
use dotenv::dotenv;

mod server;
mod models;
mod views;
mod controllers;
mod database;

fn main() {
    dotenv().ok();
    env_logger::init();
    let _ser = server::start();
}
