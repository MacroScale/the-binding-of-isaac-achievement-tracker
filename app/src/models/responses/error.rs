use serde::Serialize;

#[derive(Serialize)]
pub struct Error{
    pub status: i32, 
    pub message: String,
}
