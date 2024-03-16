use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProfileSearch{
    pub steam_id: String,
}
