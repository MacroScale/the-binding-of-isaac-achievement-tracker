use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player{
    pub steamid: String,
    pub communityvisibilitystate: i32,
    pub profilestate: i32,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub avatarhash: String,
    pub lastlogoff: i64,
    pub personastate: i32,
    pub realname: Option<String>,
    pub primaryclanid: Option<String>,
    pub timecreated: Option<i64>,
    pub personastateflags: Option<i32>,
    pub loccountrycode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSummary{
    pub response: Response,
}
