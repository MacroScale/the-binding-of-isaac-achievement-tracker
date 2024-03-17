use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Response{
    pub success: i32,
    pub steamid: Option<String>,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProfileSearch{
    pub response: Response,
}
