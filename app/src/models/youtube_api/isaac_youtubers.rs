use std::fs;
use std::io::prelude::*;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use anyhow;

use crate::models::youtube_api::youtuber::Youtuber;


#[derive(Serialize, Deserialize, Debug)]
pub struct IsaacYoutubers{
    pub status: i32,
    pub message: String,
    pub last_updated: DateTime<Utc>,
    pub mattman: Option<Youtuber>,
    pub northernlion: Option<Youtuber>,
    pub hutts: Option<Youtuber>,
    pub nyantuber: Option<Youtuber>,
    pub isaacguru: Option<Youtuber>,
    pub slayxc: Option<Youtuber>,
}


fn read_file_cache() -> anyhow::Result<String> {
    let mut file = fs::File::open("isaac_youtubers.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl IsaacYoutubers{
    pub async fn new() -> anyhow::Result<IsaacYoutubers>{
 
        let json_str = read_file_cache();
        
        if json_str.is_ok(){
            let isaac_youtubers: IsaacYoutubers = serde_json::from_str(&json_str.unwrap())?;
            // check if time is more than 1 hour
            let last_updated = isaac_youtubers.last_updated;
            let now = Utc::now();

            // if less then 24 hours
            if now.signed_duration_since(last_updated).num_hours() < 24{
                return Ok(isaac_youtubers);
            }
        }

        let last_updated = Utc::now();

        let mattman = Youtuber::new("UCGkEZAV34aWfgKqawCVgjbA").await?;
        let northernlion = Youtuber::new("UC3tNpTOHsTnkmbwztCs30sA").await?;
        let hutts = Youtuber::new("UCy3avhfHpBbbgwZpNvuVklg").await?;
        let nyantuber = Youtuber::new("UCdgr-ovS_WwdUBooTvn_evg").await?;
        let isaacguru = Youtuber::new("UC_HBRhLFbenszKiqQYu4EVQ").await?;
        let slayxc = Youtuber::new("UC6nnUx0BNRZiY3ez_BCjmUw").await?;

        let isaac_youtubers = IsaacYoutubers{
            status: 200,
            message: "success".to_string(),
            last_updated,
            mattman: Some(mattman),
            northernlion: Some(northernlion),
            hutts: Some(hutts),
            nyantuber: Some(nyantuber),
            isaacguru: Some(isaacguru),
            slayxc: Some(slayxc),
        };

        let json_str = serde_json::to_string(&isaac_youtubers)?;
        let mut file = fs::File::create("isaac_youtubers.json")?;
        file.write_all(json_str.as_bytes())?;


        return Ok(isaac_youtubers);
    }
}
