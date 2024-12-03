var _steam_data = null;

var _app_state = {
    is_tainted: false,
    current_character_id: 0,
    current_achievement_id: 0,
}

var _achievement_unlock_condition = null;

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

const dance_map = new Map([
    ["0", "/static/dance_gif/isaac.gif"],
    ["1", "/static/dance_gif/mag.gif"],
    ["2", "/static/dance_gif/cain.gif"],
    ["3", "/static/dance_gif/judas.gif"],
    ["4", "/static/dance_gif/blue_baby.gif"],
    ["5", "/static/dance_gif/eve.gif"],
    ["6", "/static/dance_gif/samson.gif"],
    ["7", "/static/dance_gif/azazel.gif"],
    ["8", "/static/dance_gif/lazarus.gif"],
    ["9", "/static/dance_gif/eden.gif"],
    ["10", "/static/dance_gif/the_lost.gif"],
    ["11", "/static/dance_gif/lilith.gif"],
    ["12", "/static/dance_gif/the_keeper.gif"],
    ["13", "/static/dance_gif/apollyon.gif"],
    ["14", "/static/dance_gif/the_forgotten.gif"],
    ["15", "/static/dance_gif/bethany.gif"],
    ["16", "/static/dance_gif/esau.gif"],
// tainted
    ["16", "/static/dance_gif/isaac.gif"],
    ["17", "/static/dance_gif/mag.gif"],
    ["18", "/static/dance_gif/cain.gif"],
    ["19", "/static/dance_gif/judas.gif"],
    ["20", "/static/dance_gif/blue_baby.gif"],
    ["21", "/static/dance_gif/eve.gif"],
    ["22", "/static/dance_gif/samson.gif"],
    ["23", "/static/dance_gif/azazel.gif"],
    ["24", "/static/dance_gif/lazarus.gif"],
    ["25", "/static/dance_gif/eden.gif"],
    ["26", "/static/dance_gif/the_lost.gif"],
    ["27", "/static/dance_gif/lilith.gif"],
    ["28", "/static/dance_gif/the_keeper.gif"],
    ["29", "/static/dance_gif/apollyon.gif"],
    ["30", "/static/dance_gif/the_forgotten.gif"],
    ["31", "/static/dance_gif/bethany.gif"],
    ["32", "/static/dance_gif/esau.gif"],
]);


function getCharacterData(character_id, characterData){
    return characterData.find(c => c.id === character_id);
}

function validateSteamData(steam_data=_steam_data){
    if (steam_data.status === 200) return true;
    return false
}

async function ProfileSearchRedeemer(req){
    let res = await req.json();
    _steam_data = processProfileSearch(res);

    if (!validateSteamData(_steam_data)){
        console.log(_steam_data);

        if (_steam_data.message === "Invalid steam id"){
            $("#modal-img").attr("src", "/static/error popups/No_Profile.png");
        }
        else if (_steam_data.message === "Failed to get player game data"){
            $("#modal-img").attr("src", "/static/error popups/No_Game_Data.png");
        }


        $(".content").css("filter", "blur(5px)");
        $(".navbar").css("filter", "blur(5px)");
        $(".modal-wrapper").show();
        $("html").css("overflow", "hidden");
        return;
    } 
    else{
        $(".content").css("filter", "none");
        $(".navbar").css("filter", "none");
        $(".modal-wrapper").hide();
        $("html").css("overflow", "auto");
    }


    console.log(_steam_data);

    createNavPlaytimeInfo();
    updatePlaytime(_steam_data.playerGameData.playtime_forever);
    updateProfilePostit(_steam_data.summary);
    updateCharacterOverlay(_app_state.current_character_id);
    updateCompletionMarks(_app_state.current_character_id);
    updateAchievements(_app_state.current_achievement_id, _steam_data.achievements);
    populateAchievementsTable(_steam_data.achievements, _steam_data.achievementImageData);
}

function processProfileSearch(res){
    if (res.status === 200) {
        let summary = res.player_summary.response.players[0] ?? {};
        let achievements = res.player_achievements.playerstats.achievements ?? [];
        let gameData = res.player_game_data ?? {};
        let characterData = res.character_data ?? [];
        let achievementImageData = res.achievement_image_data ?? [];

        return {
            status: res.status,
            message: res.message,
            summary: summary,
            playerGameData: gameData,
            achievements: achievements,
            characterData: characterData,
            achievementImageData: achievementImageData 
        }
    }

    return {
        status: res.status,
        message: res.message,
        summary: {},
        playerGameData: {},
        achievements: [],
        characterData: [],
        achievementImageData: []
    }
}

function createNavPlaytimeInfo(){
    let html = `
        <img src="/static/logo.png">
        <h2><span id="completed-achievements">0</span>/640 (<span id="completed-achievements-percentage">0</span>%)</h2>
        <h2><span id="playtime"></span> Hours</h2>
    `
    $(".nav-center").html(html);
}

function updatePlaytime(playtime){
    playtime = Math.trunc(playtime / 60);
    $("#playtime").text(playtime);
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
    let characterData = getCharacterData(character_id, _steam_data.characterData);
    let character_unlock_id = characterData.character_unlock;

    if (character_unlock_id === -1) $("#current-character").attr("src", `/static/character wheel/character icons/${characterData.unlock_url}`);
    else {
        let is_locked = _steam_data.achievements.find(a => parseInt(a.apiname) === character_unlock_id).achieved === 0;
        if (is_locked) $("#current-character").attr("src", `/static/character wheel/locked character icons/${characterData.locked_url}`);
        else $("#current-character").attr("src", `/static/character wheel/character icons/${characterData.unlock_url}`);
    }
}

function updateCompletionMarks(character_id=_app_state.current_character_id, achievements=_steam_data.achievements, is_tainted=_app_state.is_tainted){
    
    unsetCompletionMarks(_app_state.is_tainted);

    let characterData = getCharacterData(character_id, _steam_data.characterData);
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

            if (mark_id === "ALL") return setAllCompletionMarks(_app_state.is_tainted);
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



function updateAchievements(achievement_id=_app_state.current_achievement_id, achievements=_steam_data.achievements, achievementImageData=_steam_data.achievementImageData){

    let seven_page_ids= [];
    let seven_page_images = [null, null, null, null, null, null, null];

    let achievement = null; 
    let achievementImage = null;
    let achievementImageSuffix = null;
    let achievementText = null;

    let completed_achievements = null;

    let mod = (x,y) => { return ((x % y) + y) % y; }

    for (let idx = 0; idx < 7; idx++){
        let next_id = achievement_id - 3 + idx;
        next_id = mod(next_id, achievements.length);
        seven_page_ids.push(next_id+1);
    }

    for (let idx = 0; idx < achievements.length; idx++){
        let achieve = achievements[idx];
        let apiname = parseInt(achieve.apiname);

        let currentAchievementImageData = achievementImageData.find(a => parseInt(a._id) === apiname);
        let currentAchievementText = currentAchievementImageData._text;

        if (apiname === achievement_id+1) {
            achievement = achieve;
            achievementImage = currentAchievementImageData;
            achievementImageSuffix = achievementImage._gfx;
            achievementText = currentAchievementText;
        }

        if (achieve.achieved === 1) completed_achievements++;

        if (seven_page_ids.includes(apiname)){ 
            let i = seven_page_ids.indexOf(apiname);
            seven_page_images[i] = achieve;
        }

    }

    updateAchievementProgress(completed_achievements);
    updateAchievementPostIt(achievement);
    updateAchievementOverlay(achievement.achieved === 0, achievementImageSuffix);
    updateAchievementInputNumber(achievement_id);
    updateSevenPageAchievements(seven_page_images);
    updateAchievementText(achievementText)
    updateAchievementSteamIcon(achievement_id);
}

function updateDancingCharacter(character_id){
    let characterDanceGif = dance_map.get(character_id.toString());
    console.log(characterDanceGif);
    $("#dancing-character").attr("src", characterDanceGif);
}

function populateAchievementsTable(achievements, achievementImageData){

    let table = $("#achievements-listbox-table");
    table.empty();

    for (let idx = 0; idx < achievements.length; idx++){

        let achieve = achievements[idx];
        let apiname = parseInt(achieve.apiname);
        let achieved = achieve.achieved;

        let currentAchievementImageData = achievementImageData.find(a => parseInt(a._id) === apiname);
        let achievementText = currentAchievementImageData._text;

        let row_html = `
            <tr data-id="${apiname}" data-achieved="${achieved===1}" data-text="${achievementText.toLowerCase()}">
                <td class="achievement-list-img">
                    <img src="/static/achievement search/steam_icons/${apiname}.png"/>
                </td>
                <td class="achievement-list-text">${achievementText}</td>
            </tr>
        `; 
        table.append(row_html);
    }

}

function updateAchievementPostIt(achievement){
    if (achievement.achieved === 0) {
        $("#date-unlocked").text("???"); 
        $("#time-unlocked").text("???"); 
        $("#time-zone").text("???"); 
        return;
    }

    var dateUnlocked = new Date(0); // The 0 there is the key, which sets the date to the epoch
    dateUnlocked.setUTCSeconds(achievement.unlocktime);
    let stdDate = `${dateUnlocked.getDate()}/${dateUnlocked.getMonth() + 1}/${dateUnlocked.getFullYear()}`;
    let stdTime = `${dateUnlocked.getHours()}:${(dateUnlocked.getMinutes() < 10 ? "0" : "") + dateUnlocked.getMinutes()}`;
    let stdTimeZone = `${dateUnlocked.toString().split(" ")[5]}`;

    $("#date-unlocked").text(stdDate); 
    $("#time-unlocked").text(stdTime); 
    $("#time-zone").text(stdTimeZone); 
}

function updateAchievementOverlay(locked=false, suffix="locked.png"){
    if (locked) {
        $("#current-achievement").attr("src", "/static/achievements/achievements/locked.png");
        return;
    }
    $("#current-achievement").attr("src", `/static/achievements/achievements/${suffix}`);
}

function updateAchievementInputNumber(achievement_id){
    $("#achievement-page-search-input").val(achievement_id+1);
}

function updateSevenPageAchievements(seven_page_images){

    for (let idx = 0; idx < seven_page_images.length; idx++){
        let achievement = seven_page_images[idx];
        let locked = achievement.achieved === 0;

        let original_url = $(`#page-${idx+1}`).attr("src");
        let original_suffix = original_url.match(/\d+/)[0];

        let new_suffix = (original_suffix % 3) + 1;

        
        if (locked){
            $(`#page-${idx+1}`).attr("src", `/static/achievements/pages/lockedpage${new_suffix}.png`);
            continue;
        }

        $(`#page-${idx+1}`).attr("src", `/static/achievements/pages/page${new_suffix}.png`);
    }

}


function updateAchievementText(achievementText){
    $("#achievement-text").text(achievementText);
}

function updateAchievementProgress(completed_achievements){

    let percentage = Math.trunc((completed_achievements / 640) * 100);

    $("#completed-achievements").text(completed_achievements);
    $("#completed-achievements-percentage").text(percentage);

}

function insertAchievementsTable(achievement_id, achievementText, achieved){
}


async function getAchievementUnlockCondition(apiname = _app_state.current_achievement_id){

    let data = null;

    if (!_achievement_unlock_condition){
        //grab data
        let condition_data = await fetch(`/static/achievements/achievement_conditions.json`);
        data = await condition_data.json();
        _achievement_unlock_condition = data;
        console.log(data);
    }

    //get achievement text
    let target_id = apiname+1;
    console.log(target_id)
    let achievement_data = _achievement_unlock_condition.filter(achieve => achieve.achievement_id == target_id);
    console.log(achievement_data);
    let achievement_text = achievement_data[0].achievement_condition;

    //set achievement text
    $("#current-achievement-icon-text").text(achievement_text);

}


function updateAchievementSteamIcon(achievement_id){
    $("#current-achievement-icon").attr("src", `/static/achievement search/steam_icons/${achievement_id+1}.png`);
}
