var _steam_data = null;
var _app_state = {
    is_tainted: false,
    current_character_id: 16,
    current_achievement: 0,
}

async function ProfileSearchRedeemer(req){
    let res = await req.json();
    _steam_data = processProfileSearch(res);
    console.log(_steam_data);

    updateProfilePostit(_steam_data.summary);
    updateCharacterOverlay(_app_state.current_character_id);
    updateCompletionMarks(_app_state.current_character_id);
}

function processProfileSearch(res){
    if (res.status === 200) {
        let summary = res.player_summary.response.players[0];
        let achievements = res.player_achievements.playerstats.achievements;
        let characterData = res.character_data;

        return {
            status: res.status,
            message: res.message,
            summary: summary,
            achievements: achievements,
            characterData: characterData
        }
    }

    return {
        status: res.status,
        message: res.message,
        summary: {},
        achievements: [],
        characterData: []
    }
}

function getCharacterData(character_id, characterData=_steam_data.characterData){
    return characterData.find(c => c.id === character_id);
}


function updateProfilePostit(summary){
    let username = summary.personaname;
    let avatar = summary.avatarfull;
    let country_code = summary.loccountrycode;
    let offline = summary.personastate === 0;

    $("#profile-avatar").attr("src", avatar);
    $("#profile-username").text(username);
    $("#profile-online").attr("src", offline ? "/static/profile/offline.png" : "/static/profile/online.png");
    $("#profile-country-img").attr("src", `https://flagsapi.com/${country_code}/flat/64.png`);
}

function updateCharacterOverlay(character_id=_app_state.current_character_id, achievements=_steam_data.achievements){
    let characterData = getCharacterData(character_id);
    let character_unlock_id = characterData.character_unlock;

    if (character_unlock_id === -1) $("#current-character").attr("src", `/static/character wheel/character icons/${characterData.unlock_url}`);
    else {
        let is_locked = _steam_data.achievements.find(a => parseInt(a.apiname) === character_unlock_id).achieved === 0;
        if (is_locked) $("#current-character").attr("src", `/static/character wheel/locked character icons/${characterData.locked_url}`);
        else $("#current-character").attr("src", `/static/character wheel/character icons/${characterData.unlock_url}`);
    }
}


const mark_map = new Map([
    ["#heart", "/static/completion marks/completion marks/moms_heart.png"],
    ["#isaac", "/static/completion marks/completion marks/isaac.png"],
    ["#polaroid", "/static/completion marks/completion marks/polaroid.png"],
    ["#cent", "/static/completion marks/completion marks/greed.png"],
    ["#satan", "/static/completion marks/completion marks/satan.png"],
    ["#negative", "/static/completion marks/completion marks/negative.png"],
    ["#brimstone", "/static/completion marks/completion marks/mega_satan.png"],
    ["#star", "/static/completion marks/completion marks/boss_rush.png"],
    ["#hush", "/static/completion marks/completion marks/hush.png"],
    ["#knife", "/static/completion marks/completion marks/mother.png"],
    ["#dadsnote", "/static/completion marks/completion marks/beast.png"],
    ["#wrinkled-paper", "/static/completion marks/completion marks/delirium.png"],
]);

function updateCompletionMarks(character_id=_app_state.current_character_id, achievements=_steam_data.achievements, is_tainted=_app_state.is_tainted){
    
    unsetCompletionMarks();

    let characterData = getCharacterData(character_id);
    let unlock_map = new Map();

    unlock_map.set(characterData.hard_hard_all, "ALL");
    unlock_map.set(characterData.hard_cent_sign, "hard_cent");
    unlock_map.set(characterData.hard_heart, "hard_heart");

    unlock_map.set(characterData.brimstone, "#brimstone");
    unlock_map.set(characterData.cent_sign, "#cent");
    unlock_map.set(characterData.cross, "#isaac");
    unlock_map.set(characterData.dads_note, "#dadsnote");
    unlock_map.set(characterData.hushs_face, "#hush");
    unlock_map.set(characterData.inverted_cross, "#satan");
    unlock_map.set(characterData.knife, "#knife");
    unlock_map.set(characterData.negative, "#negative");
    unlock_map.set(characterData.polaroid, "#polaroid");
    unlock_map.set(characterData.star, "#star");
    unlock_map.set(characterData.wrinkled_paper, "#wrinkled-paper");

    for (let idx = 0; idx < achievements.length; idx++){
        let achievement = achievements[idx];
        let apiname = parseInt(achievement.apiname);
        let achieved = achievement.achieved;
        
        if (unlock_map.has(apiname) && achieved === 1){
            let mark_id = unlock_map.get(apiname);

            if (mark_id === "ALL") return setAllCompletionMarks();
            if (mark_id === "hard_cent") {
                $("#cent").attr("src", "/static/completion marks/completion marks/greed_hard.png");
                continue;
            }
            else if(mark_id === "hard_heart") {
                $("#heart").attr("src", "/static/completion marks/completion marks/moms_heart_hard.png");
                continue;
            }

            $(mark_id).attr("src", mark_map.get(mark_id));
        }
    }
}



function unsetCompletionMarks(is_tainted=false){
    let prefix = is_tainted ? "alt_" : "";
    $("#heart").attr("src", "/static/completion marks/completion marks/moms_heart.png")
    $("#isaac").attr("src", "/static/completion marks/completion marks/null.png")
    $("#polaroid").attr("src", "/static/completion marks/completion marks/null.png")
    $("#cent").attr("src", "/static/completion marks/completion marks/null.png")
    $("#satan").attr("src", "/static/completion marks/completion marks/null.png")
    $("#negative").attr("src", "/static/completion marks/completion marks/null.png")
    $("#brimstone").attr("src", "/static/completion marks/completion marks/null.png")
    $("#star").attr("src", "/static/completion marks/completion marks/null.png")
    $("#hush").attr("src", "/static/completion marks/completion marks/null.png")
    $("#knife").attr("src", "/static/completion marks/completion marks/null.png")
    $("#dadsnote").attr("src", "/static/completion marks/completion marks/null.png")
    $("#wrinkled-paper").attr("src", `/static/completion marks/completion marks/${prefix}post_it.png`)
}

function setAllCompletionMarks(is_tainted=false){
    let prefix = is_tainted ? "alt_" : "";
    $("#heart").attr("src", `/static/completion marks/completion marks/moms_heart_hard.png`)
    $("#isaac").attr("src", `/static/completion marks/completion marks/isaac_hard.png`)
    $("#polaroid").attr("src", `/static/completion marks/completion marks/polaroid_hard.png`)
    $("#cent").attr("src", `/static/completion marks/completion marks/greed_hard.png`)
    $("#satan").attr("src", `/static/completion marks/completion marks/satan_hard.png`)
    $("#negative").attr("src", `/static/completion marks/completion marks/negative_hard.png`)
    $("#brimstone").attr("src", `/static/completion marks/completion marks/mega_satan_hard.png`)
    $("#star").attr("src", `/static/completion marks/completion marks/boss_rush_hard.png`)
    $("#hush").attr("src", `/static/completion marks/completion marks/hush_hard.png`)
    $("#knife").attr("src", `/static/completion marks/completion marks/mother_hard.png`)
    $("#dadsnote").attr("src", `/static/completion marks/completion marks/beast_hard.png`)
    $("#wrinkled-paper").attr("src", `/static/completion marks/completion marks/${prefix}delirium_hard.png`)
}
