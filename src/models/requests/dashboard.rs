use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProfileSearch {
    pub steam_id: String,
}
